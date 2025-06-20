import React, { Component } from 'react';
import './Header.css';
import { Link, Redirect } from 'react-router-dom';
import { connect } from 'react-redux';
import { logOut, getUser } from '../../redux/reducers/authReducer';

class Header extends Component {
  componentDidMount() {
    this.props.getUser();
  }

  handleLogOut = () => {
    this.props.logOut();
  };

  render() {
    if (!this.props.user.user_id) return <Redirect to='/' />;
    return (
      <header>
        <h1 className='primary-logo'>
          <i class='fas fa-road'></i>TriPlan{' '}
        </h1>
        <ul className='primary-ul'>
          <Link to='/home'>
            <li>Home</li>
          </Link>
          <Link to='/MyTrips'>
            <li>My Trips</li>
          </Link>
          <Link>
            <li onClick={this.handleLogOut}>Log Out</li>
          </Link>
        </ul>
      </header>
    );
  }
}
const mapStateToProps = state => {
  return {
    loggedIn: state.authReducer.loggedIn,
    user: state.authReducer.user
  };
};

export default connect(mapStateToProps, {
  logOut,
  getUser
})(Header);
