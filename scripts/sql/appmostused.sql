select 
	distinct package_name,
	count(appusageevents.id) 
from  
	appusageevents
group by 
	appusageevents.package_name order by count desc;
