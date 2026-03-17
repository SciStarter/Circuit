select id, opportunity, first, latest, mode, participant, "location"
from c_involvement where participant = $1 and opportunity = $2 limit 1;
