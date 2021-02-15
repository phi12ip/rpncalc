export default function KeyPad(props) {
    return (
        <div className="calc-keypad">

            <div className="button-row">
                <button className="calc-button" name="(" onClick={e => props.onClick(e.target.name)}>(</button>
                <button className="calc-button" name="CE" onClick={e => props.onClick(e.target.name)}>CE</button>
                <button className="calc-button" name=")" onClick={e => props.onClick(e.target.name)}>)</button>
                <button className="calc-button" name="C" onClick={e => props.onClick(e.target.name)}>C</button>
            </div>

            <div className="button-row">
                <button className="calc-button" name="1" onClick={e => props.onClick(e.target.name)}>1</button>
                <button className="calc-button" name="2" onClick={e => props.onClick(e.target.name)}>2</button>
                <button className="calc-button" name="3" onClick={e => props.onClick(e.target.name)}>3</button>
                <button className="calc-button" name="+" onClick={e => props.onClick(e.target.name)}>+</button>
            </div>

            <div className="button-row">
                <button className="calc-button" name="4" onClick={e => props.onClick(e.target.name)}>4</button>
                <button className="calc-button" name="5" onClick={e => props.onClick(e.target.name)}>5</button>
                <button className="calc-button" name="6" onClick={e => props.onClick(e.target.name)}>6</button>
                <button className="calc-button" name="-" onClick={e => props.onClick(e.target.name)}>-</button>
            </div>

            <div className="button-row">
                <button className="calc-button" name="7" onClick={e => props.onClick(e.target.name)}>7</button>
                <button className="calc-button" name="8" onClick={e => props.onClick(e.target.name)}>8</button>
                <button className="calc-button" name="9" onClick={e => props.onClick(e.target.name)}>9</button>
                <button className="calc-button" name="*" onClick={e => props.onClick(e.target.name)}>x</button>
            </div>

            <div className="button-row">
                <button className="calc-button" name="." onClick={e => props.onClick(e.target.name)}>.</button>
                <button className="calc-button" name="0" onClick={e => props.onClick(e.target.name)}>0</button>
                <button className="calc-button" name="=" onClick={e => props.onClick(e.target.name)}>=</button>
                <button className="calc-button" name="/" onClick={e => props.onClick(e.target.name)}>÷</button>
            </div>

        </div>
    );
} 