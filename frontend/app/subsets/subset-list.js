import React, { Fragment, useCallback } from 'react';
import { useSolver } from '#app/solver';
import { useCases } from './store';

export const solved = {
    corners: [ [ 'U', 'L', 'B' ], [ 'U', 'B', 'R' ], [ 'U', 'R', 'F' ], [ 'U', 'F', 'L' ] ],
    edges: [ [ 'U', 'B' ], [ 'U', 'R' ], [ 'U', 'F' ], [ 'U', 'L' ] ],
};

export const blank = {
    corners: [ [ 'X', 'X', 'X' ], [ 'X', 'X', 'X' ], [ 'X', 'X', 'X' ], [ 'X', 'X', 'X' ] ],
    edges: [ [ 'X', 'X' ], [ 'X', 'X' ], [ 'X', 'X' ], [ 'X', 'X' ] ]
};

export const zbll = {
    corners: [ [ 'X', 'X', 'X' ], [ 'X', 'X', 'X' ], [ 'X', 'X', 'X' ], [ 'X', 'X', 'X' ] ],
    edges: [ [ 'U', 'X' ], [ 'U', 'X' ], [ 'U', 'X' ], [ 'U', 'X' ] ]
};

export const coSolved = {
    corners: [ [ 'U', 'X', 'X' ], [ 'U', 'X', 'X' ], [ 'U', 'X', 'X' ], [ 'U', 'X', 'X' ] ],
    edges: [ [ 'X', 'X' ], [ 'X', 'X' ], [ 'X', 'X' ], [ 'X', 'X' ] ]
};

export default function () {

    const { loadSubset } = useSolver();
    const { setLL } = useCases();

    const setSubset = (subset) => {
        return () => {
            setLL(subset);
            loadSubset({ index: '0', ...subset });
        }
    };

    return (
        <div className="subset-list">
            <button onClick={setSubset(solved)}> solved </button>
            <button onClick={setSubset(blank)}> blank </button>
            <button onClick={setSubset(zbll)}> ZBLL </button>
            <button onClick={setSubset(coSolved)}> CO solved </button>
        </div>
    );
}
