use anyhow::Error;
use chrono::Utc;
use common::{
    model::analytics::{Hosts, HostsData, HostsDataChart},
    Database, ToFixedOffset,
};

use crate::{retry_query, CommonState};

pub async fn collect(
    db: &Database,
    org: &common::model::Partner,
    state: &CommonState,
) -> Result<Hosts, Error> {
    let host_names = retry_query!(
        sqlx::query!(
            r#"
SELECT DISTINCT
  "exterior"->>'organization_name' AS "host!: String"
FROM
  c_opportunity
WHERE
  "exterior"->>'organization_name' != '' AND
  "exterior"->>'organization_name' != 'test' AND
  "created" < $1 AND
  ("exterior"->>'partner')::uuid = $2
ORDER BY
  "exterior"->>'organization_name'
"#,
            state.end,
            org.exterior.uid,
        )
        .map(|row| row.host)
        .fetch_all(db)
        .await
    )?;

    let total_hosts = host_names.len().try_into().unwrap_or(0);
    let mut total_opportunities = 0;
    let mut max = HostsDataChart::default();
    let mut hosts = Vec::with_capacity(total_hosts.try_into().unwrap_or(1024));

    for (i, host_name) in host_names.into_iter().enumerate() {
        println!(
            "{} ... [{}: {:?}] {} / {} {}",
            Utc::now(),
            org.id.unwrap_or(0),
            &org.exterior.name,
            i,
            total_hosts,
            &host_name,
        );

        let row = retry_query!(sqlx::query!(
            r#"
SELECT
  COALESCE(COUNT(*), 0) AS "total!: i64",
  COALESCE(SUM(c_opportunity_by_uid_is_current("opportunity")::int), 0) AS "live!: i64",
  COALESCE(SUM("views")::bigint, 0) AS "views!: i64",
  COALESCE(SUM(c_opportunity_by_uid_clicks_during("opportunity", $3, $4))::bigint, 0) AS "opportunity_exits!: i64",
  COALESCE(SUM(c_opportunity_by_uid_didits_during("opportunity", $3, $4))::bigint, 0) AS "didits!: i64",
  COALESCE(SUM(c_opportunity_by_uid_saves_during("opportunity", $3, $4))::bigint, 0) AS "saves!: i64",
  COALESCE(SUM(c_opportunity_by_uid_likes_during("opportunity", $3, $4))::bigint, 0) AS "likes!: i64",
  COALESCE(SUM(c_opportunity_by_uid_shares_during("opportunity", $3, $4))::bigint, 0) AS "shares!: i64",
  COALESCE(SUM(c_opportunity_by_uid_calendar_adds_during("opportunity", $3, $4))::bigint, 0) AS "calendar_adds!: i64"
FROM
  c_analytics_cache INNER JOIN c_opportunity ON
    "opportunity" = ("exterior"->>'uid')::uuid
WHERE
  "partner" = $1 AND
  "exterior"->>'organization_name' = $2 AND
  "begin" = $3 AND
  "end" = $4
"#,
            org.exterior.uid,
            &host_name,
            state.begin,
            state.end,
        )
        .fetch_one(db)
        .await)?;

        let row = HostsDataChart {
            name: Some(host_name),
            total: row.total.try_into().unwrap_or(0),
            live: row.live.try_into().unwrap_or(0),
            views: row.views.try_into().unwrap_or(0),
            opportunity_exits: row.opportunity_exits.try_into().unwrap_or(0),
            didits: row.didits.try_into().unwrap_or(0),
            saves: row.saves.try_into().unwrap_or(0),
            likes: row.likes.try_into().unwrap_or(0),
            shares: row.shares.try_into().unwrap_or(0),
            calendar_adds: row.calendar_adds.try_into().unwrap_or(0),
        };

        total_opportunities += row.total;

        max.total = max.total.max(row.total);
        max.live = max.live.max(row.live);
        max.views = max.views.max(row.views);
        max.opportunity_exits = max.opportunity_exits.max(row.opportunity_exits);
        max.didits = max.didits.max(row.didits);
        max.saves = max.saves.max(row.saves);
        max.likes = max.likes.max(row.likes);
        max.shares = max.shares.max(row.shares);
        max.calendar_adds = max.calendar_adds.max(row.calendar_adds);

        hosts.push(row);
    }

    Ok(Hosts {
        updated: Utc::now().to_fixed_offset(),
        data: HostsData {
            total_hosts,
            total_opportunities,
            max,
            hosts,
        },
    })
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
