import React, { Fragment, useState, useCallback } from 'react';
import { exploreSolve } from '#app/solver';
import LL from './ll';

// #[derive(Clone, Debug)]
// pub struct Transform {
//     pub edge_cycles: Vec<Vec<usize>>,
//     pub edge_flips: Vec<usize>,
//     pub corner_cycles: Vec<Vec<usize>>,
//     pub corner_twists: Vec<(usize, Twist)>,
//     pub centre_cycles: Vec<Vec<usize>>,
// }

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
                    /* eslint-disable */
                    const edge_cycles = [edges.map(d => d[0]).filter((d, i) => d !== i)];
                    const edge_flips = edges.filter((d) => d[1] !== 0).map(d => d[0]);
                    const corner_cycles = [corners.map(d => d[0]).filter((d, i) => d !== i)];
                    const corner_twists = corners.filter((d) => d[1] !== 0).map(d => [d[0], d[1]==1 ? 'Cw' : 'Acw']);
                    const transform = ({
                        edge_flips,
                        edge_cycles,
                        corner_cycles,
                        corner_twists,
                        centre_cycles: [],
                    });
                    /* eslint-enable */
                    exploreSolve(transform);
                }}>
                    solve
                </button>
            </div>
        </div>
    );
}
