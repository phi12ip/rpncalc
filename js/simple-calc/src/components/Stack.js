import '../Stack.css';
import React, { useState } from 'react';
import StackDisplay from './StackDisplay.js';
import StackControl from './StackControl.js';

export default function Stack() {

    const [stack, setStack] = useState({ elems: [] });

    function onClickStack({ name, value }) {
        if (name === 'push') {
            pushStack(4);
        }

        else if (name === 'pop') {
            popStack();
        }
    }

    function pushStack(val) {
        const { elems } = stack;
        elems.push(val);
        setStack({ elems })
    }

    function popStack() {
        const { elems } = stack;
        const popped = elems.pop();
        console.log(popped);
        setStack({ elems })
    }
    return (
        <>
            <h1>Simple Stack Machine</h1>
            <div className="stack">
                <StackDisplay elems={stack.elems} />
                <StackControl onClick={onClickStack} />
            </div>
        </>
    )
}