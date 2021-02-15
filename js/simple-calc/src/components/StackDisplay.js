export default function StackDisplay({elems}) {

    return (
        <div className="stack-display">
            {elems.map( (value, index) => {
                    return (<p>{value}</p>)
                })
            }
        </div>
    )
}