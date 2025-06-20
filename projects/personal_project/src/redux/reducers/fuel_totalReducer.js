import axios from 'axios';
require('dotenv').config();

//INITIAL STATE
const initialState = {
  fuel_total: []
};

//CONSTANTS
const UPDATE_STATE = 'UPDATE_STATE';
const RESET_FIELD = 'RESET_FIELD';
const GET_FUEL_TOTAL = 'GET_FUEL_TOTAL';
const ADD_FUEL_TOTAL = 'ADD_FUEL_TOTAL';
const DELETE_FUEL_TOTAL = 'DELETE_FUEL_TOTAL';
const UPDATE_FUEL_TOTAL = 'UPDATE_FUEL_TOTAL';

//ACTION CREATORS
export const updateState = e => {
  return {
    type: UPDATE_FUEL_TOTAL,
    payload: e
  };
};

export const get_fuel_total = () => {
  return {
    type: GET_FUEL_TOTAL,
    payload: axios.get('/api/user/fuel_total')
  };
};
export const add_fuel_total = fuel_total => {
  return {
    type: ADD_FUEL_TOTAL,
    payload: axios.post('/api/fuel_total', {
      fuel_total
    })
  };
};
export const delete_fuel_total = receipt => {
  return {
    type: DELETE_FUEL_TOTAL,
    payload: axios.delete('/api/fuel_total/:id')
  };
};
export const update_fuel_total = new_fuel_total => {
  return {
    type: UPDATE_FUEL_TOTAL,
    payload: new_fuel_total
  };
};
//REDUCER
export default function fuel_totalReducer(state = initialState, action) {
  const { type, payload } = action;
  console.log(action);
  switch (type) {
    case UPDATE_STATE:
      return {
        ...state,
        ...payload
      };
    case RESET_FIELD:
      return {
        fuel_total: []
      };

    //TRIPS
    case GET_FUEL_TOTAL + '_FULFILLED':
      return {
        ...state,
        fuel_total: payload.data
      };
    case ADD_FUEL_TOTAL + '_FULFILLED':
      return {
        ...state,
        fuel_total: payload.data
      };
    case DELETE_FUEL_TOTAL + '_FULFILLED':
      return {
        ...state,
        fuel_total: action.payload.data
      };
    case UPDATE_FUEL_TOTAL + '_FULFILLED':
      return {
        ...state,
        fuel_total: action.payload
      };

    default:
      return state;
  }
}
