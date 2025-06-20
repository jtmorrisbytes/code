select * from trip

inner join fuel_total
on trip.trip_id = fuel_total.trip_id

inner join receipt
on trip.receipt_id = receipt.receipt_id

where trip_id = $1;