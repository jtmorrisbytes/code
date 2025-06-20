INSERT INTO users 
(user_email, first_name, hash)
VALUES
($1, $2, $3)
RETURNING*