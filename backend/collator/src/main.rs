use chrono::{DateTime, FixedOffset};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .connect(&std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL"))
        .await
        .expect("connect to database");

    common::migrate(&pool).await?;

    sqlx::query!("SELECT c_refresh_log_by_when_this_year()")
        .execute(&pool)
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

        let mut iter = common::model::Opportunity::catalog(&pool).await?;
        while let Some(opp) = iter.get_next(&pool).await {
            opportunity::cache(&pool, &opp, temporary, begin, end).await?;
        }

        for partner_ref in common::model::partner::Partner::catalog(&pool).await? {
            let partner =
                common::model::partner::Partner::load_by_id(&pool, partner_ref.id).await?;
            organization::cache(&pool, &partner, temporary, begin, end).await?;
        }

        overview::cache(&pool, temporary, begin, end).await?;

        for status in Status::iter() {
            let state = collect(&pool, begin, end, period, status).await?;

            let mut iter = common::model::Opportunity::catalog(&pool).await?;
            while let Some(opp) = iter.get_next(&pool).await {
                opportunity::collect(&pool, &opp, &state).await?;
            }

            for partner_ref in common::model::partner::Partner::catalog(&pool).await? {
                let partner =
                    common::model::partner::Partner::load_by_id(&pool, partner_ref.id).await?;
                organization::collect(&pool, &partner, &state).await?;
            }

            overview::collect(&pool, &state).await?;
        }
    }

    ga4::clear_cached_temporary(&pool).await?;

    Ok(())
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


SNM DATA OVERVIEW

{
                "updated": "2022-07-28T14:33:27.12343242-07:00",
                "engagement": {
                    "data": {
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "search_max": 41363,
                        "stats": {
                            "unique_visitors": 5732,
                            "accounts": 1112,
                            "opportunity_views": 4214,
                            "opportunity_unique": 3214,
                            "opportunity_exits": 2341,
                            "didits": 123,
                            "saves": 632,
                            "likes": 423,
                            "shares": 343,
                            "calendar_adds": 211,
                        },
                        "searches": [
                            {"phrase": "Science Festival", "searches": 41363},
                            {"phrase": "mumblety-peg", "searches": 8123},
                            {"phrase": "kids and families", "searches": 712}
                        ],
                    },
                },

                "demographics": {
                    "sex": {
                        "male": {"index": 92, "proportion": 0.4507, "national": 0.4915},
                        "female": {"index": 108, "proportion": 0.5493, "national": 0.5085},
                    },
                    "age": {
                        "18-20": {"index": 96, "proportion": 0.0539, "national": 0.0562, "male": {"index": 108, "proportion": 0.0273, "national": 0.0253}, "female": {"index": 86, "proportion": 0.0266, "national": 0.0309}},
                        "21-24": {"index": 86, "proportion": 0.0679, "national": 0.0788, "male": {"index": 87, "proportion": 0.0307, "national": 0.0355}, "female": {"index": 86, "proportion": 0.0372, "national": 0.0433}},
                        "25-29": {"index": 135, "proportion": 0.1358, "national": 0.1005, "male": {"index": 105, "proportion": 0.0474, "national": 0.0453}, "female": {"index": 160, "proportion": 0.0884, "national": 0.0552}},
                        "30-34": {"index": 112, "proportion": 0.1149, "national": 0.1025, "male": {"index": 116, "proportion": 0.0535, "national": 0.0462}, "female":{"index": 109, "proportion": 0.0615, "national": 0.0563}},
                        "35-39": {"index": 112, "proportion": 0.1305, "national": 0.117, "male": {"index": 95, "proportion": 0.0501, "national": 0.0527}, "female": {"index": 125, "proportion": 0.0804, "national": 0.0642}},
                        "40-44": {"index": 101, "proportion": 0.1187, "national": 0.118, "male": {"index": 87, "proportion": 0.0463, "national": 0.0532}, "female": {"index": 112, "proportion": 0.0725, "national": 0.0648}},
                        "45-49": {"index": 94, "proportion": 0.1017, "national": 0.1076, "male": {"index": 104, "proportion": 0.0505, "national": 0.0485}, "female": {"index": 87, "proportion": 0.0512, "national": 0.0591}},
                        "50-54": {"index": 100, "proportion": 0.1093, "national": 0.1092, "male": {"index": 118, "proportion": 0.058, "national": 0.0492}, "female": {"index": 85, "proportion": 0.0512, "national": 0.06}},
                        "55-59": {"index": 89, "proportion": 0.058, "national": 0.0654, "male": {"index": 111, "proportion": 0.0326, "national": 0.0295}, "female": {"index": 71, "proportion": 0.0254, "national": 0.0359}},
                        "60-64": {"index": 76, "proportion": 0.0505, "national": 0.0668, "male": {"index": 92, "proportion": 0.0277, "national": 0.0301}, "female": {"index": 62, "proportion": 0.0228, "national": 0.0367}},
                        "65+": {"index": 75, "proportion": 0.0588, "national": 0.0781, "male": {"index": 75, "proportion": 0.0266, "national": 0.0352}, "female": {"index": 75, "proportion": 0.0322, "national": 0.0429}},
                    },
                    "education": {
                        "No College": {"index": 85, "proportion": 0.3642, "national": 0.4306},
                        "College": {"index": 103, "proportion": 0.4279, "national": 0.4158},
                        "Grad. Sch.": {"index": 135, "proportion": 0.2079, "national": 0.1536},
                    },
                    "income": {
                        "$0-50k": {"index": 84, "proportion": 0.3365, "national": 0.4028},
                        "$50-100k": {"index": 116, "proportion": 0.3657, "national": 0.3139},
                        "$100-150k": {"index": 114, "proportion": 0.1741, "national": 0.1532},
                        "$150k+": {"index": 95, "proportion": 0.1237, "national": 0.1301},
                    },
                    "children": {
                        "No Children under 17": {"index": 103, "proportion": 0.5247, "national": 0.5071},
                        "Some Children under 17": {"index": 96, "proportion": 0.4753, "national": 0.4929},
                    },
                    "ethnicity": {
                        "Cauc.": {"index": 98, "proportion": 0.7362, "national": 0.7506},
                        "Afr. Am.": {"index": 103, "proportion": 0.0961, "national": 0.0936},
                        "Asian": {"index": 140, "proportion": 0.0614, "national": 0.0437},
                        "Hisp": {"index": 99, "proportion": 0.0968, "national": 0.0978},
                        "Other": {"index": 66, "proportion": 0.0094, "national": 0.0143},
                    },
                },

                "states": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "max": {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        "states": {
                            'Texas': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "regional": {
                                'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                "regions": {
                                    'Agua Dulce': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-97.910833, 27.7825]},
                                    'Bear Creek': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-97.932778, 30.181944]},
                                    'Blackwell': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-100.319722, 32.085556]},
                                    'Buffalo Springs': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-101.709167, 33.532222]},
                                },
                            }},
                            'California': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "regional": {
                                'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                "regions": {
                                    'Arcata': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-124.090556, 40.868056]},
                                    'Buellton': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-120.193889, 34.614167]},
                                    'Cotati': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-122.709167, 38.327778]},
                                    'Eastvale': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-117.564167, 33.963611]},
                                },
                            }},
                            'Oregon': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "regional": {
                                'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                "regions": {
                                    'Keizer': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-123.021944, 45.000556]},
                                    'Monmouth': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-123.23, 44.849167]},
                                    'Winston': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-123.4175, 43.121667]},
                                    'Nyssa': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-116.996944, 43.879167]},
                                },
                            }},
                        },
                    },
                },

                "technology": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        'mobile': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332},
                        'tablet': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132},
                        'desktop': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                    },
                },

                "traffic": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "columns": ["Unique", "New", "Returning"],
                        "max": {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        "chart": [
                            {"date": "2022-07-29", "Unique": 15, "New": 8, "Returning": 4},
                            {"date": "2022-07-28", "Unique": 8, "New": 2, "Returning": 7},
                            {"date": "2022-07-27", "Unique": 13, "New": 11, "Returning": 1},
                        ],
                        "pie": {
                            "labels": ["Direct", "Payed Search", "Display", "Affiliates", "Other"],
                            "datasets": [{
                                "label": "Referrers by Type",
                                "hoverOffset": 4,
                                "backgroundColor": ["#387ab5", "#5da136", "#cd4c24", "#e7e93c", "#5abdda"],
                                "data": [202, 15, 11, 0, 0],
                            }],
                        },
                        "table": [
                            {"name": "Test Ref 1", "type": "Direct", "Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332},
                            {"name": "Test Ref 2", "type": "Direct", "Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132},
                            {"name": "Test Ref 3", "type": "Direct", "Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                            {"name": "Test Ref 4", "type": "Paid Search", "Unique Users": 3, "New Users": 34, "Returning Users": 32, "Total Pageviews": 23, "Unique Pageviews": 22, "Avg. Time": 32},
                            {"name": "Test Ref 5", "type": "Paid Search", "Unique Users": 12, "New Users": 14, "Returning Users": 32, "Total Pageviews": 23, "Unique Pageviews": 32, "Avg. Time": 12},
                            {"name": "Test Ref 6", "type": "Display", "Unique Users": 11, "New Users": 13, "Returning Users": 33, "Total Pageviews": 22, "Unique Pageviews": 32, "Avg. Time": 13},
                        ],
                    },
                },

                "crossover": {
                    "engagement_types": ["Views", "Unique", "Clicks to Website"],
                    "data": {
                        "engagement_type": "Views",
                        "chart": {
                            "citizen_science": {
                                "proportion": 0.23,
                                "live_science": {"Views": 0.6, "Unique": 0.5, "Clicks to Website": 0.333},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.05, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.166, "Unique": 0.04, "Clicks to Website": 0.333},
                                "policy": {"Views": 0.166, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.4, "Clicks to Website": 0.333},
                                "formal_education": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications": {"Views": 0.166, "Unique": 0.01, "Clicks to Website": 0.0},
                                "unspecified": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                            "live_science": {
                                "proportion": 0.05,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "museum_or_science_center": {
                                "proportion": 0.16,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "maker": {
                                "proportion": 0.21,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "policy": {
                                "proportion": 0.08,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "out_of_school_time_program": {
                                "proportion": 0.22,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "formal_education": {
                                "proportion": 0.01,
                                "citizen_science": {"Views": .1, "Unique": 0.0, "Clicks to Website": 0.0},
                                "live_science": {"Views": 0.1, "Unique": 0.0, "Clicks to Website": 0.0},
                                "museum_or_science_center": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "policy": {"Views": 0.1, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "unspecified": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                            "science_communications": {
                                "proportion": 0.04,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "unspecified": {
                                "proportion": 0.0,
                                "citizen_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "live_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "museum_or_science_center": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "policy": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "formal_education": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications,": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                        },
                    },
                },
            }
 */
