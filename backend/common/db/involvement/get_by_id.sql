select id, opportunity, first, latest, mode, participant, "location"
from c_involvement where id = $1 limit 1;
