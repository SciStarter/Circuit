select id, opportunity, partner, "when", mode, keywords, participant, snml, "location"
from c_participation where id = $1 limit 1;
