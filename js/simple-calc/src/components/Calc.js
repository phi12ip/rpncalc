import React, { useState } from 'react';

import CalcResult from './CalcResult.js';
import CalcKeyPad from './CalcKeyPad.js';

import '../Calc.css';

export default function Calc() {
  const [calc, setCalc] = useState({ result: "" });

    function onClick(button) {

        if (button === "=") {
          calculate()
        }
    
        else if (button === "C") {
          reset()
        }
        else if (button === "CE") {
          backspace()
        }
    
        else {
          setCalc({
            result: calc.result + button
          })
        }
      }
    
      function calculate() {
        try {
          setCalc({
            // eslint-disable-next-line
            result: (eval(calc.result) || "") + ""
          })
        } catch (e) {
          console.error(e)
          setCalc({
            result: "error"
          })
    
        }
      }
    
      function reset() {
        setCalc({
          result: ""
        })
      }
    
      function backspace() {
        setCalc({
          result: calc.result.slice(0, -1)
        })
      }

    return (
        <>
            <h1>Simple Calculator</h1>
            <div className="calc">
                <CalcResult result={calc.result} />
                <CalcKeyPad onClick={onClick} />
            </div>
        </>
    )
}
