-- selecting a single receipt by receipt_id
-- SELECT * FROM receipt
-- WHERE receipt_id = $1;

-- or

-- selecting receipt(s) from a trip
SELECT * FROM receipt
WHERE trip_id = $1;

