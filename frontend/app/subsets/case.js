import React, { Fragment, useState } from 'react';

import LL from './ll';

const auf = ['','U ','U2 ','U\' '];

function getName(solution) {
    return (
        <span className="blue">
            {solution.invert?'invert ':''}{solution.mirror?'mirror ':''}
            {solution.name}
        </span>
    );
}

function Moves({ data }) {
    const { solution } = data;
    const [showName, setName] = useState(true);
    return (
        <span
            onClick={() => { setName(!showName); }}
            className="moves"
        >
            {auf[solution[0]]}
            {showName ? getName(solution[1]) : (
                <span className="blue">{solution[1].moves}</span>
            )}
            {solution.length > 2 && <Fragment>
                {'\n'}{auf[solution[2]]}
                {showName ? getName(solution[3]) :  (
                    <span className="blue">{solution[3].moves}</span>
                )}
            </Fragment>}
        </span>
    );
}

export default function Case({ case_, solutions }) {
    return (
        <div
            className="case visible"
        >
            <LL case_={case_} />
            <br />
            <pre>
                {solutions[0] && <Moves data={solutions[0]} />}
                {false && solutions.map((data, i) => (
                    <Fragment key={i}>
                        <Moves data={data} />
                        <hr />
                    </Fragment>
                ))}
            </pre>
            {solutions.length} solutions
        </div>
    )
}
