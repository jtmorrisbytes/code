UPDATE fuel_stops
SET fuel_stops = $1

WHERE 
    fuel_stops_id = $2;

SELECT * FROM fuel_stops;