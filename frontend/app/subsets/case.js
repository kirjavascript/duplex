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
            {solution.length === 4 && <Fragment>
                {'\n'}{auf[solution[2]]}
                {showName ? getName(solution[3]) :  (
                    <span className="blue">{solution[3].moves}</span>
                )}
            </Fragment>}
        </span>
    );
}

function findSolution(solutions) {
    // TODO: improve
    // if just one, just choose which has the shortest moves
    const stars = solutions.filter(({solution}) => {
        return solution.length === 2;
    });
    const ranked = (stars.length ? stars : solutions).map((data) => {
        const { solution: [auf0, solution0, auf1, solution1 = {}] } = data;
        const aufs = +(auf0>0&&1) + +(auf1>0&&1);
        const transforms = solution0.mirror + solution0.invert
            + solution1.mirror + solution1.invert;

        return {
            // score,
            // length,
            weight: transforms + aufs,
            data,
        };
    }).sort((a, b) => a.weight - b.weight);

    // order
    // filter
    return {
        stars: !!stars.length,
        best: ranked.length ? ranked[0].data : undefined,
    };
}
// 1217172485964883

export default function Case({ case_, solutions }) {

    const { best } = findSolution(solutions);

    return (
        <div
            className="case visible"
        >
            <LL case_={case_} />
            <br />
            <pre>
                {best && <Moves data={best} />}
                {false && solutions.map((data, i) => (
                    <Fragment key={i}>
                        <Moves data={data} />
                        <hr />
                    </Fragment>
                ))}
            </pre>
            {solutions.length} solutions
            <pre>
                {JSON.stringify(findSolution(solutions).solution, 0, 4)}
            </pre>
            <div className="index">{case_.index}</div>
        </div>
    )
}
