select id, opportunity, first, latest, mode, participant, "location"
from c_involvement where opportunity = $1;
