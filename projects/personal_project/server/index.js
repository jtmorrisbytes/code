require('dotenv').config();
const express = require('express');
const massive = require('massive');
const session = require('express-session');
const app = express();
const { SERVER_PORT, CONNECTION_STRING, SESSION_SECRET } = process.env;
// const { trip } = require('./controllers/tripController');

//CONTROLLERS
const auth = require('./controllers/authController');
const trip = require('./controllers/tripController');
const receipt = require('./controllers/receiptController');
const fuel_total = require('./controllers/fuel_total_Controller');
const fuel_stops = require('./controllers/fuel_stops_Controller');

massive(CONNECTION_STRING)
  .then(db => {
    app.set('db', db);
    console.log('DB CONNECTED');
  })
  .catch(res => console.log(res));

app.use(
  session({
    saveUninitialized: true,
    resave: false,
    secret: SESSION_SECRET,
    cookie: {
      maxAge: 1000 * 60 * 60 * 24 * 7
    }
  })
);

app.use(express.json());

//authentication endpoints
app.post('/auth/register', auth.register);
app.post('/auth/login', auth.login);
app.get('/auth/logout', auth.logOut);
app.get('/auth/user', auth.get_user);
//trips
app.get('/api/trips', trip.getTrips);
app.get('/api/user/trips', trip.getUserTrips);
app.delete('/api/trip/:id', trip.deleteTrip);
app.post('/api/trip', trip.addTrip);
app.put('/api/trip', trip.put_trip);
//receipts
app.get('./api/user/receipt', receipt.getReceipt);
app.post('/api/user/receipt/:id', receipt.addReceipt);
app.delete('/api/receipt/:id', receipt.deleteReceipt);
//fuel_trips
app.get('/api/user/fuel_total', fuel_total.get_fuel_total);
app.post('/api/fuel_total', fuel_total.add_fuel_total);
app.delete('/api/fuel_total/:id', fuel_total.delete_fuel_trip);
//fuel_trips
app.get('/api/user/fuel_stops', fuel_stops.get_fuel_stops);
app.post('/api/fuel_stops', fuel_stops.get_fuel_stops);
app.delete('/api/fuel_stops/:id', fuel_stops.get_fuel_stops);

app.listen(SERVER_PORT, () => console.log(`the server in on!`));
