WITH opportunity_stats AS (
  SELECT
    opp_partner AS partner_uid,
    COUNT(*) AS total_opportunities,
    COUNT(*) FILTER (WHERE c_opportunity_is_current(c_opportunity) = true) AS current_opportunities,
    COUNT(*) FILTER (WHERE c_opportunity_is_current_as_of(c_opportunity, NOW() - INTERVAL '1 month') = true) AS current_opportunities_one_month_ago
  FROM c_opportunity
  GROUP BY opp_partner
)
SELECT DISTINCT
  c_partner."name" AS "name",
  COALESCE(c_partner.contact->>'name', c_partner.manager->>'name', '') AS "contact_name",
  COALESCE(c_partner.contact->>'email', c_partner.manager->>'email', '') AS "contact_email",
  COALESCE(os.total_opportunities, 0) AS "total_opportunities",
  COALESCE(os.current_opportunities, 0) AS "current_opportunities",
  COALESCE(os.current_opportunities_one_month_ago, 0) AS "current_opportunities_one_month_ago"
FROM c_partner
LEFT JOIN opportunity_stats os ON os.partner_uid = c_partner.uid
WHERE EXISTS (
  SELECT 1 FROM c_person
  WHERE c_person.join_channel = 'Exchange' AND c_person.join_channel_detail = c_partner.uid
)
ORDER BY c_partner."name" ASC;
