import React, { Fragment } from 'react';
import { Link } from 'react-router-dom';
import { useSolutions } from '#app/solver/store';
import { useCases } from './store';
import Case from './case';
import Renderer from './renderer';
import Select from './select';
import SubsetList from './subset-list';

export default function Subsets() {
    const { cases, subset, select, setSelect } = useCases();
    const { solutions, length: coverage } = useSolutions();
    const hasSubset = subset.length > 0;


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
                            <Link to="/trainer">
                                {' '}view in trainer
                            </Link>
                        </div>
                    )}
                    <SubsetList />
                    <select
                        value={select}
                        onChange={(e) => {
                            setSelect(e.target.value);
                        }}
                    >
                        <option value="transform">least transforms</option>
                        <option value="length">shortest movecount</option>
                    </select>
                </div>
            </div>

            <Renderer caseList={caseList}>
                {(obj) => <Case {...obj} />}
            </Renderer>
        </div>
    );
}
