CREATE OR REPLACE FUNCTION to_date(date bigint) RETURNS varchar(20) AS $$
        BEGIN
                RETURN to_char(to_timestamp(date / 1000), 'DD/MM/YYYY HH24:IM:SS');
        END;
$$ LANGUAGE plpgsql;
