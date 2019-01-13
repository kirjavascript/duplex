import React, { Fragment } from 'react';
import { useSolutions } from '#app/solver/store';
import { useCases } from './store';
import Case from './case';
import Renderer from './renderer';

export default function Subsets() {
    const { cases } = useCases();
    const { solutions, length: coverage } = useSolutions();


    // TODO: AUF, change colour
    // star cases that use a single alg
    // trim AUF
    // select solution
    // starred first
    // reduce auf / transforms / movecount
    // inverse / mirror weight 0..4


    const caseList = cases.map((case_) => ({
        case_,
        solutions: solutions[case_.index] || [],
    }));

    return (
        <div className="subsets">
            <div className="info">
                <span className="data">
                    {cases.length}
                </span>
                cases
                <span className="data">
                    {cases.length - coverage}
                </span>
                unsolved
                <span className="data">
                    {cases.length ? Math.round((coverage/cases.length)*100) : 0}%
                </span>
                coverage
            </div>

            <Renderer caseList={caseList}>
                {({ case_, solutions }) => (
                    <Case
                        key={case_.index}
                        case_={case_}
                        solutions={solutions[case_.index] || []}
                    />
                )}
            </Renderer>

            <div className="cases">
                {false && cases.slice(0, 100).map((case_, i) => {
                    return (
                        <Case
                            key={case_.index}
                            case_={case_}
                            solutions={solutions[case_.index] || []}
                        />
                    )
                })}
            </div>
        </div>
    );
}
