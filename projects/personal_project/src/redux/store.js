import { createStore, combineReducers, applyMiddleware } from 'redux';
import promise from 'redux-promise-middleware';
import authReducer from './reducers/authReducer';
import tripReducer from './reducers/tripReducer';
import receiptReducer from './reducers/receiptReducer';
import fuel_totalReducer from './reducers/fuel_totalReducer';
import fuel_stopsReducer from './reducers/fuel_stopsReducer';

const root = combineReducers({
  authReducer,
  tripReducer,
  receiptReducer,
  fuel_totalReducer,
  fuel_stopsReducer
});

export default createStore(root, applyMiddleware(promise));
