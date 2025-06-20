INSERT INTO fuel_total
    (fuel_total, trip_id)
VALUES
    ($1, $2)
RETURNING*;