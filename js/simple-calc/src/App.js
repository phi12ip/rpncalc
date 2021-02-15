import React, { useState } from 'react';

import './App.css';

import Calc from './components/Calc.js';
import Stack from './components/Stack.js';


function App() {
  return (
    <div>
      <div className="container">
        <Calc/>
      </div>
      <div className="container">
        <Stack/>
      </div>
    </div>
  );
}
export default App;
