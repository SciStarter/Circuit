use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Duration, FixedOffset, Utc};
use common::{
    model::analytics::{AbsoluteTimePeriod, EngagementDataBar, RelativeTimePeriod, Status},
    Database,
};
use sqlx::postgres::PgPoolOptions;
use strum::IntoEnumIterator;
use uuid::Uuid;

mod ga4;
mod ga4_bigquery;
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
  AVG("view_count")::bigint AS "mean_views: i64",
  AVG("unique_count")::bigint AS "mean_unique: i64",
  PERCENTILE_CONT(0.5) WITHIN GROUP (order by "view_count")::bigint AS "median_views: i64",
  PERCENTILE_CONT(0.5) WITHIN GROUP (order by "unique_count")::bigint AS "median_unique: i64"
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
            row.mean_views.unwrap_or(0).try_into().unwrap_or(0),
            row.mean_unique.unwrap_or(0).try_into().unwrap_or(0),
            row.median_views.unwrap_or(0).try_into().unwrap_or(0),
            row.median_unique.unwrap_or(0).try_into().unwrap_or(0),
        )
    })
    .fetch_one(db)
    .await?;

    let (mean_clicks, median_clicks) = sqlx::query!(
        r#"
SELECT
  AVG("clicks")::bigint AS "mean_clicks: i64",
  PERCENTILE_CONT(0.5) WITHIN GROUP (order by "clicks")::bigint AS "median_clicks: i64"
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
            row.mean_clicks.unwrap_or(0).try_into().unwrap_or(0),
            row.median_clicks.unwrap_or(0).try_into().unwrap_or(0),
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

async fn process(db: &Database, cycle: u64) -> Result<(), Box<dyn std::error::Error>> {
    let threshold: DateTime<FixedOffset> = DateTime::from_str("2021-01-01T00:00:00Z").unwrap();

    sqlx::query!("SELECT c_refresh_log_by_when_this_year()")
        .execute(db)
        .await?;

    for period in RelativeTimePeriod::iter() {
        println!("{} [{}][{:?}]", Utc::now(), cycle, &period);

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

        let mut opp_tasks = Vec::new();

        let mut iter = common::model::Opportunity::catalog(db).await?;
        while let Some(opp) = iter.get_next(db).await {
            opp_tasks.push(tokio::spawn({
                let db = db.clone();
                async move {
                    println!(
                        "{} [{}][{:?}][{}: {:?}] Caching opp",
                        Utc::now(),
                        cycle,
                        &period,
                        opp.id.unwrap_or(0),
                        &opp.exterior.title
                    );
                    opportunity::cache(&db, &opp, temporary, begin, end).await?;
                    Result::<(), anyhow::Error>::Ok(())
                }
            }))
        }

        let opp_tasks_total = opp_tasks.len();
        for (i, opp_fut) in opp_tasks.into_iter().enumerate() {
            let _ = opp_fut.await?;
            println!(
                "{} [{}][{:?}] Finished caching {} / {} opportunities",
                Utc::now(),
                cycle,
                &period,
                i,
                opp_tasks_total,
            );
        }

        for partner_ref in common::model::partner::Partner::catalog(db).await? {
            let partner = common::model::partner::Partner::load_by_id(db, partner_ref.id).await?;
            println!(
                "{} [{}][{:?}][{}: {:?}] Caching partner",
                Utc::now(),
                cycle,
                &period,
                partner.id.unwrap_or(0),
                &partner.exterior.name
            );
            organization::cache(db, &partner, temporary, begin, end).await?;
        }

        println!("{} [{}][{:?}] Caching overview", Utc::now(), cycle, &period);
        overview::cache(db, temporary, begin, end).await?;

        for status in Status::iter() {
            println!(
                "{} [{}][{:?}][{:?}] Collecting status",
                Utc::now(),
                cycle,
                &period,
                &status
            );
            let state = collect(db, begin, end, period, status).await?;

            let mut iter = common::model::Opportunity::catalog(db).await?;
            while let Some(opp) = iter.get_next(db).await {
                if !opp.current_as_of(&threshold) {
                    continue;
                }

                println!(
                    "{} [{}][{:?}][{:?}][{}: {:?}] Collecting opportunity",
                    Utc::now(),
                    cycle,
                    &period,
                    &status,
                    opp.id.unwrap_or(0),
                    &opp.exterior.title
                );
                let data = opportunity::collect(db, &opp, &state).await?;

                sqlx::query!(
                    r#"
INSERT INTO c_analytics_compiled (
  "about",
  "kind",
  "period",
  "status",
  "data"
) VALUES (
  $1, 0, $2, $3, $4
)
ON CONFLICT ("about", "kind", "period", "status") DO UPDATE SET
  "data" = EXCLUDED."data"
"#,
                    opp.exterior.uid,
                    period.discriminate(),
                    status.discriminate(),
                    serde_json::to_value(data)?,
                )
                .execute(db)
                .await?;
            }

            let mut partner_tasks = Vec::new();

            for partner_ref in common::model::partner::Partner::catalog(db).await? {
                partner_tasks.push(tokio::spawn({
                    let db = db.clone();
                    let state = state.clone();
                    async move {
                        let partner =
                            common::model::partner::Partner::load_by_id(&db, partner_ref.id)
                                .await?;
                        println!(
                            "{} [{}][{:?}][{:?}][{}: {:?}] Collecting partner",
                            Utc::now(),
                            cycle,
                            &period,
                            &status,
                            partner.id.unwrap_or(0),
                            &partner.exterior.name
                        );

                        let data = organization::collect(&db, &partner, &state).await?;

                        sqlx::query!(
                            r#"
INSERT INTO c_analytics_compiled (
  "about",
  "kind",
  "period",
  "status",
  "data"
) VALUES (
  $1, 0, $2, $3, $4
)
ON CONFLICT ("about", "kind", "period", "status") DO UPDATE SET
  "data" = EXCLUDED."data"
"#,
                            partner.exterior.uid,
                            period.discriminate(),
                            status.discriminate(),
                            serde_json::to_value(data)?,
                        )
                        .execute(&db)
                        .await?;

                        println!(
                            "{} [{}][{:?}][{:?}][{}: {:?}] Collecting hosts",
                            Utc::now(),
                            cycle,
                            &period,
                            &status,
                            partner.id.unwrap_or(0),
                            &partner.exterior.name
                        );
                        let data = hosts::collect(&db, &partner, &state).await?;

                        sqlx::query!(
                            r#"
INSERT INTO c_analytics_compiled (
  "about",
  "kind",
  "period",
  "status",
  "data"
) VALUES (
  $1, 1, $2, $3, $4
)
ON CONFLICT ("about", "kind", "period", "status") DO UPDATE SET
  "data" = EXCLUDED."data"
"#,
                            partner.exterior.uid,
                            period.discriminate(),
                            status.discriminate(),
                            serde_json::to_value(data)?,
                        )
                        .execute(&db)
                        .await?;

                        println!(
                            "{} [{}][{:?}][{:?}][{}: {:?}] Partner collected",
                            Utc::now(),
                            cycle,
                            &period,
                            &status,
                            partner.id.unwrap_or(0),
                            &partner.exterior.name
                        );

                        Result::<(), anyhow::Error>::Ok(())
                    }
                }))
            }

            for part_fut in partner_tasks {
                let _ = part_fut.await?;
            }

            println!(
                "{} [{}][{:?}][{:?}] Collecting overview",
                Utc::now(),
                cycle,
                &period,
                &status,
            );
            let data = overview::collect(db, &state).await?;

            sqlx::query!(
                r#"
INSERT INTO c_analytics_compiled (
  "about",
  "kind",
  "period",
  "status",
  "data"
) VALUES (
  $1, 0, $2, $3, $4
)
ON CONFLICT ("about", "kind", "period", "status") DO UPDATE SET
  "data" = EXCLUDED."data"
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

    let mut cycle = 0;

    loop {
        let began = Utc::now();

        if let Err(err) = process(&pool, cycle).await {
            println!("{:?}", err);
        };

        let finished = Utc::now();

        let took = finished - began;

        println!(
            "Analytics pass took {} days, {:02} hours, {:02} minutes, {:02} seconds",
            took.num_days(),
            took.num_hours() % 24,
            took.num_minutes() % 60,
            took.num_seconds() % 60
        );

        cycle += 1;

        let throttle = Duration::days(7);

        if took < throttle {
            tokio::time::sleep((throttle - took).to_std()?).await;
        }
    }
}
