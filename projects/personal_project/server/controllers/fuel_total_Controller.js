//FUNCTION TO ENDPOINTS IN INDEX.JS
//GETTING INFO FROM DATABASE THROUGH SQL FILES

//GET
// const get_fuel_total = (req, res) => {
//   const db = req.app.get('db');
//   db.trip
//     .get_fuel_total()
//     .then(fuel_total => res.status(200).json(fuel_total))
//     .catch(err => console.log(err));
// };
const get_fuel_total = (req, res) => {
  const db = req.app.get('db');
  const { fuel_total_id } = req.body;
  db.fuel_total
    .get_fuel_total(fuel_total_id)
    .then(response => res.status(200).json(response))
    .catch(err => console.log(err));
};
//POST
const add_fuel_total = (req, res) => {
  const db = req.app.get('db');
  const { fuel_total } = req.body;
  db.fuel_total
    .post_fuel_total(fuel_total)
    .then(added_fuel_trip => res.status(200).json(added_fuel_trip))
    .catch(err => console.log(err));
};
//DELETE
const delete_fuel_trip = (req, res) => {
  const db = req.app.get('db');
  const trip_id = +req.param.trip_id;

  db.delete_fuel_trip(trip_id).then(response => {
    res.status(200).json(response);
  });
};
//PUT
const update_fuel_trip = (req, res) => {
  const db = req.app.get('db');
  const { fuel_total } = req.body;
  const fuel_trip_id = +req.params.fuel_trip_id;

  db.updateTrip([fuel_total, fuel_total_id]).then(response => {
    res.status(200).json(response);
  });
};

module.exports = {
  get_fuel_total,
  get_fuel_total,
  add_fuel_total,
  delete_fuel_trip,
  // post_fuel_total,
  update_fuel_trip
};
