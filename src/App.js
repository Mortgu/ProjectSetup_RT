import logo from './logo.svg';
import './App.css';

import { invoke } from '@tauri-apps/api/tauri';
import React from 'react';

const invokee = window.__TAURI_IPC__.invoke;
invoke('my_custom_command', { invokeMessage: "Hello!" }).then((message) => console.log(message))

function App() {


  return (
    <React.Fragment>
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <p>
            Edit <code>src/App.js</code> and save to reload.
          </p>
        </header>
      </div>
    </React.Fragment>
  );
}

export default App;
