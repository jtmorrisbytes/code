-- trip_id
-- user_id
-- milage
-- fuel_total
-- origin
-- destination

INSERT INTO users 
(user_email, first_name, hash)
VALUES
('testing', 'testing', 'test')
RETURNING*

delete from users
where user_id > 0 

select * from fuel_stops;


//////////////////
||DUMMY DATA FOR TRIP TABLE||

INSERT INTO trip 
(user_id, milage, fuel_total, origin, destination)
values
(19, 234, 50.20, 'NY', 'TX')
(20, 234, 50.79, 'CA', 'RI')
(21, 234, 80.20, 'RI', 'PA')
(22, 234, 62.40, 'FL', 'TX')
(23, 234, 150.22, 'NC', 'MA')
(24, 234, 20.20, 'NY', 'TX')
(25, 234, 35.20, 'MA', 'CT');
////////////////////////
SELECT * from trip;
WHERE user_id = $1
////////////////////////
create table fuel_total(fuel_trip_id serial primary key,
fuel_total numeric);
///////////////////////
create table milage (milage_id serial primary key,
milage numeric);
///////////////////////
alter table fuel_total
add column trip_id int REFERENCES trip(trip_id)

update table fuel_total
set trip_id = 1;

select * from trip
inner join fuel_total
on trip.trip_id = fuel_total.trip_id
/////////////////////////
create table receipt (receipt_id serial primary key,
milage text);
////////////////////////
alter table receipt
add column trip_id int REFERENCES trip(trip_id);
///////////////////////
alter table trip
add column 
fuel_stops_id int REFERENCES fuel_stops(fuel_stops_id);
////////////////////////
CREATE TABLE fuel_stops(fuel_stops_id serial PRIMARY KEY,
  stop_location character varying
);

