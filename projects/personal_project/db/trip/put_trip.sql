-- UPDATE trip
-- SET
--     (origin, destination, fuel_total_id, milage_id) = ($2, $3, $4...)
-- WHERE
--     (user_id) = $1;

-- UPDATE receipt
-- SET 
-- column1 = $5, column2 = $6

-- UPDATE fuel_table
-- set 
-- column1 = $7, column2 = $8

-- SELECT * FROM trip;

UPDATE trip
SET
    (origin, destination, fuel_total_id, receipt_id, milage, fuel_stops_id) = ($2, $3, $4, $5, $6, $7)
WHERE
    (user_id) = $1;

UPDATE fuel_total
SET
    column1 = $8, column2 = $9
UPDATE receipt
SET
    column1 = $10, column2 = $11
UPDATE fuel_stops
SET
    column1 = $12, column2 = $13
SELECT * FROM trip;