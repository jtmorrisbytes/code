INSERT INTO receipt
    (receipt, trip_id)
VALUES
    ($1, $2);

SELECT * FROM receipt;