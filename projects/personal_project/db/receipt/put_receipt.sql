UPDATE receipt
SET receipt = $1

WHERE 
    receipt_id = $2;

SELECT * FROM receipt;