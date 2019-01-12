import React, { Fragment } from 'react';

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
    const showName = true;
    // TODO: tooltip hover of actual moves?
    return (
        <Fragment>
            {auf[solution[0]]}
            {showName ? getName(solution[1]) : solution[1].moves}
            {solution.length > 2 && <Fragment>
                {'\n'}{auf[solution[2]]}
                {showName ? getName(solution[3]) : solution[3].moves}
            </Fragment>}
        </Fragment>
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
