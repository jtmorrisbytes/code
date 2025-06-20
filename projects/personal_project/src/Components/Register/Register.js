import React, { Component } from 'react';
import './Register.css';
import { Link, Redirect } from 'react-router-dom';
import {
  updateState,
  resetFields,
  registerUser,
  getUser
} from '../../redux/reducers/authReducer';
import { connect } from 'react-redux';

export class Register extends Component {
  handleChange = e => {
    this.props.updateState({ [e.target.name]: e.target.value });
    console.log('register update state', e.target.value);
  };

  handleClickRegister = () => {
    console.log('handleClickRegister register comp line 19');
    this.props
      .registerUser(
        this.props.user_email,
        this.props.first_name,
        this.props.password
      )
      // .then(() => {
      //   this.props.loginUser(this.props.first_name, t this.props.password);
      // })
      .catch(error => {
        console.log(error);
      });
  };

  render() {
    return (
      <div>
        {this.props.loggedIn ? (
          <Redirect to='/Home/' />
        ) : (
          <div id='register-root'>
            <header>
              <nav>
                <h1 className='primary-logo'>
                  <i class='fas fa-road'></i>TriPlan
                </h1>
              </nav>
            </header>
            <div className='title-rya'>
              <h1>Register your account</h1>
            </div>
            <form>
              <h3>First Name</h3>
              <input
                type='text'
                name='first_name'
                placeholder='Enter your full name'
                onChange={this.handleChange}
              />
              <h3>Email address</h3>
              <input
                type='text'
                name='user_email'
                value={this.props.user_email}
                placeholder='Enter valid email address'
                onChange={this.handleChange}
              />
              <h3>Password</h3>
              <input
                type='password'
                name='password'
                placeholder='Enter your password'
                onChange={this.handleChange}
              />
              <div className='btn-box'>
                <button onClick={this.handleClickRegister}>Join</button>
              </div>
              <div className='signIn-box'>
                <h6>already have an account?</h6>
                <Link to='/'>
                  <h6 className='sign-link'>Sign in</h6>
                </Link>
              </div>
            </form>
          </div>
        )}
      </div>
    );
  }
}
const mapStateToProps = store => {
  return {
    first_name: store.authReducer.first_name,
    password: store.authReducer.password,
    user_email: store.authReducer.user_email,
    loggedIn: store.authReducer.loggedIn
  };
};
export default connect(mapStateToProps, {
  updateState,
  resetFields,
  registerUser,
  getUser
})(Register);
