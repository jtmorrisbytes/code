//FUNCTION TO ENDPOINTS IN INDEX.JS
//GETTING INFO FROM DATABASE THROUGH SQL FILES

//GET

const get_fuel_stops = (req, res) => {
  const db = req.app.get('db');
  const { fuel_stops_id } = req.body;
  db.fuel_stops
    .get_fuel_stops(fuel_stops_id)
    .then(response => res.status(200).json(response))
    .catch(err => console.log(err));
};
//POST
const add_fuel_stops = (req, res) => {
  const db = req.app.get('db');
  const { fuel_stops } = req.body;
  db.post_fuel_total(fuel_stops)
    .then(added_fuel_stops => res.status(200).json(added_fuel_stops))
    .catch(err => console.log(err));
};
//DELETE
const delete_fuel_stops = (req, res) => {
  const db = req.app.get('db');
  const trip_id = +req.param.trip_id;

  db.delete_fuel_stops(trip_id).then(response => {
    res.status(200).json(response);
  });
};
//PUT
const update_fuel_stops = (req, res) => {
  const db = req.app.get('db');
  const { fuel_stops } = req.body;
  const fuel_stops_id = +req.params.fuel_stops_id;

  db.updateTrip([fuel_stops, fuel_total_id]).then(response => {
    res.status(200).json(response);
  });
};

module.exports = {
  get_fuel_stops,
  add_fuel_stops,
  delete_fuel_stops,
  update_fuel_stops
};
