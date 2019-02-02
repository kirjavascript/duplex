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

function findSolution(solutions = []) {
    const stars = solutions.filter(({solution}) => {
        return solution.length === 2;
    });
    const ranked = (stars.length ? stars : solutions).map((data) => {
        const {
            solution: [auf0, solution0, auf1, solution1 = { length: 0 }],
        } = data;

        const weight = (!!solution0.mirror + !!solution0.invert
                + !!solution1.mirror + !!solution1.invert);
            // : solution0.length + +!!auf0 + +!!auf1 + +solution1.length;

        return { weight, data };
    }).sort((a, b) => a.weight - b.weight);

    const best = ranked.length ? ranked[0].data : undefined;

    return best;
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
        chosen: findSolution(solutions[case_.index]),
    }));

    const subsetCoverage = filtered.filter(case_ => solutions[case_.index]).length;

    if (select === 'first-alg') {
        caseList.sort((a, b) => {
            if (!a.chosen || !b.chosen) {
                return 0;
            } else if (a.chosen.solution[1].name < b.chosen.solution[1].name) {
                return -1;
            } else if (a.chosen.solution[1].name > b.chosen.solution[1].name) {
                return 1;
            }
            return 0;
        });
    }


    // TODO: replace filter with sorting: canonical / first alg / first alg (reduce)

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
                        <option value="canonical">canonical</option>
                        <option value="first-alg">group by alg</option>
                        <option value="first-alg-reduce">group by alg (fewer groups)</option>
                    </select>
                </div>
            </div>

            <Renderer caseList={caseList}>
                {(obj) => <Case {...obj} />}
            </Renderer>
        </div>
    );
}
