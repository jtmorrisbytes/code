import React, { Component } from 'react';
import { HashRouter } from 'react-router-dom';
import routes from './routes';

class App extends Component {
  render() {
    return (
      <HashRouter>
        <div>{routes}</div>
      </HashRouter>
    );
  }
}

export default App;
