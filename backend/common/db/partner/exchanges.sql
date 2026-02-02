WITH opportunity_stats AS (
  SELECT
    exterior->'partner' AS partner_uid,
    COUNT(*) AS total_opportunities,
    COUNT(*) FILTER (WHERE c_opportunity_is_current(interior, exterior) = true) AS current_opportunities,
    COUNT(*) FILTER (WHERE c_opportunity_is_current_as_of(interior, exterior, NOW() - INTERVAL '1 month') = true) AS current_opportunities_one_month_ago
  FROM c_opportunity
  GROUP BY exterior->'partner'
)
SELECT DISTINCT
  c_partner.exterior->>'name' AS "name",
  COALESCE(c_partner.interior->'contact'->>'name', c_partner.interior->'manager'->>'name', '') AS "contact_name",
  COALESCE(c_partner.interior->'contact'->>'email', c_partner.interior->'manager'->>'email', '') AS "contact_email",
  COALESCE(os.total_opportunities, 0) AS "total_opportunities",
  COALESCE(os.current_opportunities, 0) AS "current_opportunities",
  COALESCE(os.current_opportunities_one_month_ago, 0) AS "current_opportunities_one_month_ago"
FROM c_partner
LEFT JOIN opportunity_stats os ON os.partner_uid = c_partner.exterior->'uid'
WHERE EXISTS (
  SELECT 1 FROM c_person
  WHERE c_person.interior->'join_channel'->>'Exchange' = c_partner.exterior->>'uid'
)
ORDER BY c_partner.exterior->>'name' ASC;
