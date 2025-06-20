import axios from 'axios';
require('dotenv').config();

//INITIAL STATE
const initialState = {
  trips: []
};

//CONSTANTS
const UPDATE_STATE = 'UPDATE_STATE';
const RESET_FIELD = 'RESET_FIELD';
const GET_TRIPS = 'GET_TRIPS';
const ADD_TRIP = 'ADD_TRIP';
const DELETE_TRIP = 'DELETE_TRIP';
const UPDATE_TRIP = 'UPDATE_TRIP';

//ACTION CREATORS
export const updateState = e => {
  return {
    type: UPDATE_STATE,
    payload: e
  };
};

export const getTrips = () => {
  return {
    type: GET_TRIPS,
    payload: axios.get('/api/user/trips')
  };
};
export const addTrip = newTrip => {
  console.log(newTrip);
  return {
    type: ADD_TRIP,
    payload: axios.post('/api/trip', {
      newTrip
    })
  };
};
export const deleteTrip = trip_id => {
  return {
    type: DELETE_TRIP,
    payload: axios.delete(`/api/trip/${trip_id}`)
  };
};
export const updateTrip = newTrip => {
  return {
    type: UPDATE_TRIP,
    //axios request with url, paramater would be what you are wanting to update
    payload: newTrip
  };
};

//REDUCER
export default function reducer(state = initialState, action) {
  const { type, payload } = action;
  switch (type) {
    case UPDATE_STATE:
      return {
        ...state,
        ...payload
      };
    case RESET_FIELD:
      return {
        trips: []
      };

    //TRIPS
    case GET_TRIPS + '_FULFILLED':
      return {
        ...state,
        trips: payload.data
      };
    case ADD_TRIP + '_FULFILLED':
      console.log(payload.data);
      return {
        ...state,
        trips: payload.data
      };
    case DELETE_TRIP + '_FULFILLED':
      return {
        ...state,
        trips: action.payload.data
      };
    case UPDATE_TRIP + '_FULFILLED':
      return {
        ...state,
        trips: action.payload.data
      };

    default:
      return state;
  }
}
