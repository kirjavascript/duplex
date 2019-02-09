import React, { Fragment, useEffect, useState, useRef } from 'react';
import { Link } from 'react-router-dom';
import debounce from 'lodash/debounce';
import { useSolver } from '#app/solver';
import { useCases } from './store';
import Case from './case';
import Renderer from './renderer';
import Select from './select';
import SubsetList from './subset-list';
import LL from './ll';

export function Unsolved() {

    const { cases, solutions } = useCases();

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
    const { cases, sort, setSort, ll } = useCases();
    const { loadSubset } = useSolver();

    const coverage = cases.filter(d => d.solutionIndices).length;

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
                    <Link to="/subsets/unsolved" className="link data">
                        view unsolved cases
                    </Link>
                    <SubsetList />
                    <select
                        value={sort}
                        onChange={(e) => {
                            setSort(e.target.value);
                            loadSubset({ sort: e.target.value, ll });
                        }}
                    >
                        <option value="canonical">canonical</option>
                        <option value="group-algs">group by alg</option>
                        <option value="group-reduce">reduce groups</option>
                    </select>
                </div>
            </div>

            <Renderer caseList={cases}>
                {(obj) => <Case
                    {...obj}
                />}
            </Renderer>
        </div>
    );
}
