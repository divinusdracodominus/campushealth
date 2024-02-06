\copy appusageevents(package_name,kind,platform_type,date)
FROM '/cardinal/personal_projects/education/mHealth/processor/scripts/outputs.csv'
DELIMITER ','
CSV HEADER;
Code language: SQL (Structured Query Language) (sql)