select
	minute,package_name,body
from
	appusageevents,
	(select 
		lag(smsdata.date) 
		over 
		(order by smsdata.date asc), 
		smsdata.date,
		substring(body from 1 for 20) as body
	from smsdata) as S
where
	appusageevents.date between S.lag and S.date;
