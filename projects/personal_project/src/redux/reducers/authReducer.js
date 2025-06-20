import axios from 'axios';

const initialState = {
  first_name: '',
  password: '',
  user_email: '',
  user: {},
  loggedIn: false
};

// constants
const REGISTER_USER = 'REGISTER_USER';
const LOGIN_USER = 'LOGIN_USER';
const LOGOUT_USER = 'LOGOUT_USER';
const GET_USER = 'GET_USER';
const UPDATE_STATE = 'UPDATE_STATE';
const RESET_FIELDS = 'RESET_FIELDS';

export const updateState = e => {
  return {
    type: UPDATE_STATE,
    payload: e
  };
};

export const resetFields = () => {
  return {
    type: RESET_FIELDS
  };
};

export const registerUser = (user_email, first_name, password) => {
  return {
    type: REGISTER_USER,
    payload: axios.post('/auth/register', {
      user_email,
      first_name,
      password
    })
  };
};

export const loginUser = (user_email, password) => {
  return {
    type: LOGIN_USER,
    payload: axios.post('/auth/login', {
      user_email,
      password
    })
  };
};

export const logOut = () => {
  return {
    type: LOGOUT_USER,
    payload: axios.get('/auth/logout')
  };
};

export const getUser = () => {
  return {
    type: GET_USER,
    payload: axios.get('/auth/user')
  };
};

export default function authReducer(state = initialState, action) {
  const { type, payload } = action;
  switch (type) {
    case UPDATE_STATE:
      return {
        ...state,
        ...payload
      };
    case RESET_FIELDS:
      return {
        ...state
      };
    case `${REGISTER_USER}_PENDING`:
      return {
        ...state,
        loading: true
      };
    case `${REGISTER_USER}_FULFILLED`:
      return {
        ...state,
        loading: false,
        loggedIn: true
      };
    case `${LOGIN_USER}_PENDING`:
      return {
        ...state,
        loading: true
      };
    case `${LOGIN_USER}_FULFILLED`:
      return {
        ...state,
        loading: false,
        loggedIn: true,
        user: payload.data
      };
    case `${LOGOUT_USER}_PENDING`:
      return {
        ...state,
        loading: true
      };
    case `${LOGOUT_USER}_FULFILLED`:
      return {
        first_name: '',
        password: '',
        user_email: '',
        user: [],
        loggedIn: false
      };
    case `${GET_USER}_PENDING`:
      return {
        ...state,
        loading: true
      };
    case `${GET_USER}_FULFILLED`:
      return {
        ...state,
        loading: false,
        // loggedIn: true,
        user: payload.data
      };
    default:
      return state;
  }
}
