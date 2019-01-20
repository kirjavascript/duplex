import React, { Fragment, useState } from 'react';
import { useSolver } from '#app/solver';
import LL from './ll';

// useLL

// just make a sticker picker

export default function Select() {

    const { loadSubset } = useSolver();

    const [ll, setLL] = useState({
        corners: [ [ 'U', 'L', 'B' ], [ 'U', 'B', 'R' ], [ 'U', 'R', 'F' ], [ 'U', 'F', 'L' ] ],
        edges: [ [ 'U', 'B' ], [ 'U', 'R' ], [ 'U', 'F' ], [ 'U', 'L' ] ],
    });

    return (
        <Fragment>
            <LL
                case_={ll}
                width={280}
                height={280}
                onClick={({ type, perm, orient }) => {
                    setLL(ll => {
                        ll[type][perm][orient] = 'X';
                        return ll;
                    });
                }}
            />
            <button onClick={() => {
                loadSubset({ index: '0', ...ll });
            }}>
                get subset
            </button>
        </Fragment>
    );
}
