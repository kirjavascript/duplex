import React, { Fragment } from 'react';
import { Link } from 'react-router-dom';
import { useSolutions } from '#app/solver/store';
import { useCases } from './store';
import Case from './case';
import Renderer from './renderer';
import Select from './select';
import SubsetList from './subset-list';
import LL from './ll';

export function Unsolved() {

    const { cases  } = useCases();
    const { solutions } = useSolutions();

    const unsolved = cases.filter(case_ => !solutions[case_.index]);

    const hasSolved = unsolved.length > 3900;

    return (
        <div className="unsolved">
            {hasSolved ? (
                <span className="blue">
                    solving...
                </span>
            ) : (
                unsolved.map((case_, index) => {
                    return (
                        <LL
                            case_={case_}
                            key={index}
                            width={200}
                            height={200}
                        />
                    );
                })
            )}
        </div>
    );
}

export default function Subsets() {
    const { cases, subset, select, setSelect } = useCases();
    const { solutions } = useSolutions();
    const hasSubset = subset.length > 0;
    const coverage = Object.keys(solutions).length;

    const filtered = cases.filter((case_) => {
        return !hasSubset || subset.includes(case_.index);
    });

    const caseList = filtered.map((case_) => ({
        case_,
        solutions: solutions[case_.index] || [],
    }));

    const subsetCoverage = filtered.filter(case_ => solutions[case_.index]).length;

    // TODO: render index

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
                    </div>
                    {hasSubset && (
                        <div>
                            <span className="data">
                                {subset.length}
                            </span>
                            in subset
                            <span className="data">
                                {subset.length - subsetCoverage}
                            </span>
                            unsolved
                        </div>
                    )}
                    <Link to="/subsets/unsolved" className="link data">
                        view unsolved cases
                    </Link>
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
