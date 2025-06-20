import React, { Component } from 'react';
import './AddTrip.css';
import { connect } from 'react-redux';
import { addTrip } from '../../redux/reducers/tripReducer';
import { add_fuel_total } from '../../redux/reducers/fuel_totalReducer';

class AddTrip extends Component {
  constructor() {
    super();
    this.state = {
      origin: '',
      fuel_stop: '',
      fuel_stops: [],
      fuel_expenses: [],
      total_spent: 0,
      destination: '',
      total_miles: 0
    };
  }

  handleInput = e => {
    this.setState({ [e.target.name]: e.target.value });
  };

  handleAddFuelStop = () => {
    if (this.state.fuel_stop) {
      const fuelStopsCopy = [...this.state.fuel_stops];

      fuelStopsCopy.push(this.state.fuel_stop);

      this.setState({
        fuel_stop: '',
        fuel_stops: fuelStopsCopy
      });
    }
  };

  handleFuelExpenses = () => {
    if (this.state.total_spent) {
      const fuelTotalCopy = [...this.state.fuel_expenses];

      fuelTotalCopy.push(this.state.total_spent);

      this.setState({
        total_spent: 0,
        fuel_expenses: fuelTotalCopy.map(val => +val)
      });
    }
  };

  handleSave = () => {
    const {
      origin,
      fuel_stops,
      fuel_expenses,
      total_spent,
      destination,
      total_miles
    } = this.state;

    const newTrip = {
      origin,
      fuel_stops,
      fuel_expenses,
      total_spent,
      destination,
      total_miles
    };
    this.props.addTrip(newTrip);
    // this.props.add_fuel_total(this.state.fuel_expenses);
    console.log(newTrip);
  };
  render() {
    console.log(this.state.fuel_stop);
    console.log(this.state.fuel_stops);
    return (
      <div id='addTrip-root'>
        <div className='trip-info'>
          <h5>ORIGIN</h5>
          <input
            name='origin'
            onChange={this.handleInput}
            placeholder='Where did you start your trip?'
          />

          <div className='total-box'>
            <h5>FUEL STOPS</h5>
            <i
              name='icon'
              onClick={this.handleAddFuelStop}
              className='fas fa-plus'
            ></i>
          </div>
          <input
            name='fuel_stop'
            onChange={this.handleInput}
            value={this.state.fuel_stop}
            placeholder='where did you fuel (locations)?'
          />

          <div className='total-box2'>
            <h5>TOTAL SPENT</h5>
            <i
              name='icon'
              onClick={this.handleFuelExpenses}
              className='fas fa-plus'
            ></i>
          </div>

          <input
            name='total_spent'
            onChange={this.handleInput}
            value={this.state.total_spent}
            placeholder='how much did you spent?'
          />

          <h5>RECEIPT</h5>
          <input name='receipts' type='file' className='upload-btn' />

          <h5>DESTINATION</h5>
          <input
            name='destination'
            onChange={this.handleInput}
            placeholder='where did you end you trip?'
          />

          <h5>TOTAL MILES</h5>
          <input
            name='total_miles'
            onChange={this.handleInput}
            placeholder='how many miles were driven?'
          />

          <button onClick={this.handleSave} className='save-btn'>
            save trip
          </button>
        </div>
      </div>
    );
  }
}

const mapStateToProps = initialState => {
  return {
    trips: initialState.tripReducer.trips
  };
};

export default connect(mapStateToProps, { addTrip, add_fuel_total })(AddTrip);
