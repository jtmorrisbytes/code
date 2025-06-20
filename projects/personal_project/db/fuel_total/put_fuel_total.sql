UPDATE fuel_total
SET fuel_total = $1

WHERE 
    fuel_total_id = $2;

SELECT * FROM fuel_total;