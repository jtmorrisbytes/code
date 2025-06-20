import React, { Component } from 'react';
import './Mytrips.css';
import Header from '../Header/Header';
import AddTrip from '../AddTrip/AddTrip';
import DisplayTrip from '../DisplayTrip/DisplayTrip';

class MyTrips extends Component {
  constructor() {
    super();
    this.state = {
      view: 'displayTrip'
      //trips : []
    };
  }

  //componentdidmount  - axios request to get all trips for specific user. store in state.

  addTrip = () => {
    this.setState({ view: 'addTrip' });
  };

  displayTrip = () => {
    this.setState({ view: 'displayTrip' });
  };

  render() {
    return (
      <div className='mytrips-root'>
        <Header />
        <div className='trip-box'>
          <h1>You don't have any trips yet!</h1>
          <button onClick={this.addTrip} className='add-btn'>
            Add trip
          </button>
        </div>
        {/* {this.state.trips.length === 0 ? null : <DisplayTrip trips={this.state.trips}/>} */}
        <DisplayTrip />
        {this.state.view === 'addTrip' ? <AddTrip /> : null}
        <div className='count-box'>
          <button>count all miles</button>
          <button>count total fuel</button>
        </div>
      </div>
    );
  }
}

export default MyTrips;
