INSERT INTO fuel_stops
    (fuel_stops, trip_id)
VALUES
    ($1, $2);

SELECT * FROM fuel_stops;