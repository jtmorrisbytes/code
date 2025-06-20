import axios from 'axios';
require('dotenv').config();

//INITIAL STATE
const initialState = {
  receipts: []
};

//CONSTANTS
const UPDATE_STATE = 'UPDATE_STATE';
const RESET_FIELD = 'RESET_FIELD';
const GET_RECEIPT = 'GET_RECEIPT';
const ADD_RECEIPT = 'ADD_RECEIPT';
const DELETE_RECEIPT = 'DELETE_RECEIPT';
const UPDATE_RECEIPT = 'UPDATE_RECEIPT';

//ACTION CREATORS
export const updateState = e => {
  return {
    type: UPDATE_RECEIPT,
    payload: e
  };
};

export const getReceipt = () => {
  return {
    type: GET_RECEIPT,
    payload: axios.get('/api/user/receipt')
  };
};
export const addReceipt = receipt => {
  return {
    type: ADD_RECEIPT,
    payload: axios.post('/api/receipt/:id', {
      receipt
    })
  };
};
export const deleteReceipt = receipt => {
  return {
    type: DELETE_RECEIPT,
    payload: axios.delete('/api/receipt/:id')
  };
};
export const updateReceipt = newReceipt => {
  return {
    type: UPDATE_RECEIPT,
    payload: newReceipt
  };
};

//REDUCER
export default function reducer(state = initialState, action) {
  const { type, payload } = action;
  // console.log(action);
  switch (type) {
    case UPDATE_STATE:
      return {
        ...state,
        ...payload
      };
    case RESET_FIELD:
      return {
        receipts: []
      };

    //TRIPS
    case GET_RECEIPT + '_FULFILLED':
      return {
        ...state,
        receipts: payload.data
      };
    case ADD_RECEIPT + '_FULFILLED':
      return {
        ...state,
        receipts: payload.data
      };
    case DELETE_RECEIPT + '_FULFILLED':
      return {
        ...state,
        receipts: action.payload.data
      };
    case UPDATE_RECEIPT + '_FULFILLED':
      return {
        ...state,
        receipts: action.payload
      };

    default:
      return state;
  }
}
