select 
	distinct package_name, 
	first_name,
	last_name,
	email, 
	count(appusageevents.id) 
from 
	participants, 
	metadata, 
	appusageevents 
where 
	metadata.id = appusageevents.metadata_id 
and 
	participants.id = metadata.participant_id 
group by 
	appusageevents.package_name,
	first_name,
	last_name,
	email 
order by count desc;
