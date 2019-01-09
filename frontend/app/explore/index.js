import React, { Fragment, useState, useCallback } from 'react';
// import { exploreSolve } from '#app/solver';
import LL, { llToCube } from './ll';

export default function Explore() {
    const [edges, setEdges] = useState([[0, 0], [1, 0], [2, 0], [3, 0]]);
    const [corners, setCorners] = useState([[0, 0], [1, 0], [2, 0], [3, 0]]);

    return (
        <div className="explore">
            <LL
                edges={edges}
                corners={corners}
                onFlip={(a) => {
                    setEdges(edges => {
                        edges[a][1] ^= 1; // eslint-disable-line
                        return edges
                    });
                }}
                onTwist={(a) => {
                    setCorners(corners => {
                        corners[a][1] = (corners[a][1] + 1) % 3;
                        return corners;
                    });
                }}
                onSwapEdges={(a, b) => {
                    setEdges(edges => {
                        edges[a] = edges.splice(b, 1, edges[a])[0]; // eslint-disable-line
                        return edges;
                    });
                }}
                onSwapCorners={(a, b) => {
                    setCorners(corners => {
                        corners[a] = corners.splice(b, 1, corners[a])[0]; // eslint-disable-line
                        return corners;
                    });
                }}
            />
            <div className="actions">
                <input type="text" placeholder="specify a LL alg" />
                <button type="button">
                    load alg
                </button>
                <button type="button" onClick={() => {
                    // exploreSolve(llToCube({ corners, edges }));
                }}>
                    solve
                </button>
            </div>
        </div>
    );
}
