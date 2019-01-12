import React, { Fragment } from 'react';
import { useSolutions } from '#app/solver/store';
import { useCases } from './store';
import Case from './case';

export default function Subsets() {
    const { cases } = useCases();
    const { solutions, length: coverage } = useSolutions();

// TODO: AUF, change colour
    // star cases that use a single alg
    // trim AUF
    // select solution

    return (
        <Fragment>
            {cases.length} cases {cases.length - coverage} unsolved {Math.round((coverage/cases.length)*100)}% coverage<br />
            {cases.slice(0, 100).map((case_, i) => {
                return (
                    <Case
                        key={case_.index}
                        case_={case_}
                        solutions={solutions[case_.index]}
                    />
                )
            })}
        </Fragment>
    );
}
