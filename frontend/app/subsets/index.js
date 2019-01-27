import React, { Fragment } from 'react';
import { useSolutions } from '#app/solver/store';
import { useCases } from './store';
import Case from './case';
import Renderer from './renderer';
import Select from './select';
import SubsetList from './subset-list';

export default function Subsets() {
    const { cases, subset } = useCases();
    const { solutions, length: coverage } = useSolutions();
    const hasSubset = subset.length > 0;

    // TODO: AUF
    // show coverage of subset
    // star cases that use a single alg
    // trim AUF
    // select solution
    // starred first
    // reduce auf / transforms / movecount
    // inverse / mirror weight 0..4
    // hide cases you've already seen

    const filtered = cases.filter((case_) => {
        return !hasSubset || subset.includes(case_.index);
    });

    const caseList = filtered.map((case_) => ({
        case_,
        solutions: solutions[case_.index] || [],
    }));

    return (
        <div className="subsets">
            <div className="config">
                <Select />
                <div className="info">
                    <div>
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
                    {hasSubset && (
                        <div>
                            <span className="data">
                                {subset.length}
                            </span>
                            in subset
                        </div>
                    )}
                    <SubsetList />
                </div>
            </div>

            <Renderer caseList={caseList}>
                {(obj) => <Case {...obj} />}
            </Renderer>
        </div>
    );
}
