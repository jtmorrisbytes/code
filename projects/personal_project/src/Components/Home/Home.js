import React, { Component } from 'react';
import Header from '../Header/Header';
import './Home.css';
import { connect } from 'react-redux';

class Home extends Component {
  render() {
    return (
      <div className='home-root'>
        <Header />
        <div className='use-inf'>
          <div className='greeting-title'>
            <h2>Hello</h2>
            <h1>
              {this.props.first_name
                ? this.props.first_name.toUpperCase()
                : null}
            </h1>
          </div>
          <div className='web-des'>
            <h5>
              welcome to triPlan the website that will easily help you keep
              <br />
              <br />
              truck of your milage, locations, expenses and more while on the
              road.
              <br />
              <br />
              are you ready?
            </h5>
          </div>
          <div className='user-direct'>
            <h2>Let's Roll BABE!</h2>
          </div>
        </div>
      </div>
    );
  }
}

const mapStateToProps = state => {
  return {
    first_name: state.authReducer.user.first_name,
    loggedIn: state.authReducer.loggedIn
  };
};

export default connect(mapStateToProps)(Home);
