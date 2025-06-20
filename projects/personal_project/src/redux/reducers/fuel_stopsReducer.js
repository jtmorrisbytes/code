import axios from 'axios';
require('dotenv').config();

//INITIAL STATE
const initialState = {
  fuel_stops: []
};

//CONSTANTS
const UPDATE_STATE = 'UPDATE_STATE';
const RESET_FIELD = 'RESET_FIELD';
const GET_FUEL_STOPS = 'GET_FUEL_STOPS';
const ADD_FUEL_STOPS = 'ADD_FUEL_STOPS';
const DELETE_FUEL_STOPS = 'DELETE_FUEL_STOPS';
const UPDATE_FUEL_STOPS = 'UPDATE_FUEL_STOPS';

//ACTION CREATORS
export const updateState = e => {
  return {
    type: UPDATE_FUEL_STOPS,
    payload: e
  };
};

export const get_fuel_stops = () => {
  //   console.log(get_fuel_stops);
  return {
    type: GET_FUEL_STOPS,
    payload: axios.get('/api/user/fuel_stops')
  };
};
export const add_fuel_stops = fuel_stops => {
  return {
    type: ADD_FUEL_STOPS,
    payload: axios.post('/api/fuel_stops', {
      fuel_stops
    })
  };
};
export const delete_fuel_stops = receipt => {
  return {
    type: DELETE_FUEL_STOPS,
    payload: axios.delete('/api/fuel_stops/:id')
  };
};
export const update_fuel_stops = new_fuel_stops => {
  return {
    type: UPDATE_FUEL_STOPS,
    payload: new_fuel_stops
  };
};
//REDUCER
export default function fuel_stopsReducer(state = initialState, action) {
  const { type, payload } = action;
  switch (type) {
    case UPDATE_STATE:
      return {
        ...state,
        ...payload
      };
    case RESET_FIELD:
      return {
        fuel_stops: []
      };

    //TRIPS
    case GET_FUEL_STOPS + '_FULFILLED':
      return {
        ...state,
        fuel_stops: payload.data
      };
    case ADD_FUEL_STOPS + '_FULFILLED':
      return {
        ...state,
        fuel_stops: payload.data
      };
    case DELETE_FUEL_STOPS + '_FULFILLED':
      return {
        ...state,
        fuel_stops: action.payload.data
      };
    case UPDATE_FUEL_STOPS + '_FULFILLED':
      return {
        ...state,
        fuel_stops: action.payload
      };

    default:
      return state;
  }
}
