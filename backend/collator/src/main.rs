use chrono::{DateTime, Duration, FixedOffset, Utc};
use common::{
    model::analytics::{AbsoluteTimePeriod, EngagementDataBar, RelativeTimePeriod, Status},
    Database,
};
use sqlx::postgres::PgPoolOptions;
use strum::IntoEnumIterator;
use uuid::Uuid;

mod ga4;
mod hosts;
mod opportunity;
mod organization;
mod overview;
mod reportiter;

#[derive(Default, Debug, Clone)]
pub struct CommonState {
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    period: RelativeTimePeriod,
    status: Status,
    engagement_mean: EngagementDataBar,
    engagement_median: EngagementDataBar,
}

async fn collect(
    db: &Database,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    period: RelativeTimePeriod,
    status: Status, // Only used for constructing the result; the collected analytics are universal
) -> Result<CommonState, Box<dyn std::error::Error>> {
    let (mean_views, mean_unique, median_views, median_unique) = sqlx::query!(
        r#"
SELECT
  AVG("view_count")::bigint AS "mean_views!: i64",
  AVG("unique_count")::bigint AS "mean_unique!: i64",
  PERCENTILE_CONT(0.5) WITHIN GROUP (order by "view_count")::bigint AS "median_views!: i64",
  PERCENTILE_CONT(0.5) WITHIN GROUP (order by "unique_count")::bigint AS "median_unique!: i64"
FROM (
  SELECT "opportunity", SUM("views") AS "view_count", SUM("total_users") AS "unique_count"
  FROM c_analytics_cache
  WHERE "begin" = $1 AND "end" = $2
  GROUP BY "opportunity"
) AS c_sub
"#,
        begin,
        end
    )
    .map(|row| {
        (
            row.mean_views.try_into().unwrap_or(0),
            row.mean_unique.try_into().unwrap_or(0),
            row.median_views.try_into().unwrap_or(0),
            row.median_unique.try_into().unwrap_or(0),
        )
    })
    .fetch_one(db)
    .await?;

    let (mean_clicks, median_clicks) = sqlx::query!(
        r#"
SELECT
  AVG("clicks")::bigint AS "mean_clicks!: i64",
  PERCENTILE_CONT(0.5) WITHIN GROUP (order by "clicks")::bigint AS "median_clicks!: i64"
FROM (
  SELECT "object", COUNT(*) AS "clicks"
  FROM c_log
  WHERE "action" = 'external' AND "when" >= $1 AND "when" < $2
  GROUP BY "object"
) AS c_sub
"#,
        begin,
        end
    )
    .map(|row| {
        (
            row.mean_clicks.try_into().unwrap_or(0),
            row.median_clicks.try_into().unwrap_or(0),
        )
    })
    .fetch_one(db)
    .await?;

    Ok(CommonState {
        begin,
        end,
        period,
        status,
        engagement_mean: EngagementDataBar {
            views: mean_views,
            unique: mean_unique,
            clicks: mean_clicks,
        },
        engagement_median: EngagementDataBar {
            views: median_views,
            unique: median_unique,
            clicks: median_clicks,
        },
    })
}

async fn process(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!("SELECT c_refresh_log_by_when_this_year()")
        .execute(db)
        .await?;

    for period in RelativeTimePeriod::iter() {
        let temporary = match period {
            RelativeTimePeriod::ThisMonth => true,
            RelativeTimePeriod::LastMonth => false,
            RelativeTimePeriod::ThisQuarter => true,
            RelativeTimePeriod::LastQuarter => false,
            RelativeTimePeriod::ThisSemiannum => true,
            RelativeTimePeriod::LastSemiannum => false,
            RelativeTimePeriod::ThisYear => true,
            RelativeTimePeriod::LastYear => false,
            RelativeTimePeriod::AllTime => true,
        };

        let AbsoluteTimePeriod { begin, end } = period.absolute();

        let mut iter = common::model::Opportunity::catalog(db).await?;
        while let Some(opp) = iter.get_next(db).await {
            opportunity::cache(db, &opp, temporary, begin, end).await?;
        }

        for partner_ref in common::model::partner::Partner::catalog(db).await? {
            let partner = common::model::partner::Partner::load_by_id(db, partner_ref.id).await?;
            organization::cache(db, &partner, temporary, begin, end).await?;
        }

        overview::cache(db, temporary, begin, end).await?;

        for status in Status::iter() {
            let state = collect(db, begin, end, period, status).await?;

            let mut iter = common::model::Opportunity::catalog(db).await?;
            while let Some(opp) = iter.get_next(db).await {
                let data = opportunity::collect(db, &opp, &state).await?;

                sqlx::query!(
                    r#"
INSERT INTO c_analytics_compiled (
  "about",
  "period",
  "status",
  "data"
) VALUES (
  $1, $2, $3, $4
)
"#,
                    opp.exterior.uid,
                    period.discriminate(),
                    status.discriminate(),
                    serde_json::to_value(data)?,
                )
                .execute(db)
                .await?;
            }

            for partner_ref in common::model::partner::Partner::catalog(db).await? {
                let partner =
                    common::model::partner::Partner::load_by_id(db, partner_ref.id).await?;

                let data = organization::collect(db, &partner, &state).await?;

                sqlx::query!(
                    r#"
INSERT INTO c_analytics_compiled (
  "about",
  "period",
  "status",
  "data"
) VALUES (
  $1, $2, $3, $4
)
"#,
                    partner.exterior.uid,
                    period.discriminate(),
                    status.discriminate(),
                    serde_json::to_value(data)?,
                )
                .execute(db)
                .await?;
            }

            let data = overview::collect(db, &state).await?;

            sqlx::query!(
                r#"
INSERT INTO c_analytics_compiled (
  "about",
  "period",
  "status",
  "data"
) VALUES (
  $1, $2, $3, $4
)
"#,
                Uuid::nil(),
                period.discriminate(),
                status.discriminate(),
                serde_json::to_value(data)?,
            )
            .execute(db)
            .await?;
        }
    }

    ga4::clear_cached_temporary(db).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .connect(&std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL"))
        .await
        .expect("connect to database");

    common::migrate(&pool).await?;

    loop {
        let began = Utc::now();

        process(&pool).await?;

        let finished = Utc::now();

        let took = finished - began;

        println!("Completed one analytics pass in {}", took);

        let throttle = Duration::days(7);

        if took < throttle {
            tokio::time::sleep((throttle - took).to_std()?).await;
        }
    }
}

/*
HOSTS EXPLORER
            {
                "updated": "2022-07-28T14:33:27.12343242-07:00",
                "data": {
                    "total_hosts": 43,
                    "total_opportunities": 4212,
                    "max": {"total": 10345, "live": 10345, "views": 23442, "opportunity_exits": 2313, "didits": 1321, "saves": 1332, "likes": 1331, "shares": 433, "calendar_adds": 132},
                    "hosts": [
                        {"name": "Nerd Nite Atlanta", "total": 10345, "live": 10345, "views": 23442, "opportunity_exits": 2313, "didits": 1321, "saves": 1332, "likes": 1331, "shares": 433, "calendar_adds": 132},
                        {"name": "ASDF 01", "total": 10332, "live": 10331, "views": 23397, "opportunity_exits": 2290, "didits": 1298, "saves": 1219, "likes": 1208, "shares": 400, "calendar_adds": 109},
                        {"name": "ASDF 02", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 03", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 04", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 05", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 06", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 07", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 08", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 09", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 10", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 11", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 12", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 13", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 14", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 15", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 16", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 17", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 18", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 19", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 20", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 21", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 22", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 23", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 24", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 25", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 26", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 27", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 28", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 29", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 30", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 31", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 32", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 33", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 34", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 35", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 36", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 37", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 38", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 39", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 40", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 41", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 42", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                    ],
                },
            },


 */
