import React, { Fragment, useState, useCallback } from 'react';

function rotate(arr, n) {
    return arr.slice(n, arr.length).concat(arr.slice(0, n));
}

export function llToCube({ edges, corners }) {
    const uEdges = edges.map(edge => {
        const face = 'BRFL'[edge[0]];
        return edge[1] ? [face, 'U'] : ['U', face];
    });
    const uCorners = corners.map(corner => (
        rotate([["U","L","B"],["U","B","R"],["U","R","F"],["U","F","L"]][corner[0]], -corner[1])
    ));
    return ({
        "edges":[...uEdges,["B","L"],["B","R"],["F","R"],["F","L"],["D","B"],["D","R"],["D","F"],["D","L"]],
        "corners":[...uCorners,["D","B","L"],["D","R","B"],["D","F","R"],["D","L","F"]],
        "centres":["U","D","B","R","F","L"]
    });
}

const [yellow, red, green, blue, orange] = [
    '#FFFF5A', '#FA2222', '#90EE90', 'steelblue', '#FAA222',
];

const getSideColor = (index) => (
    [red, blue, orange, green][index%4]
);

export default function LL({
    edges,
    corners,
    onSwapEdges,
    onSwapCorners,
    onFlip,
    onTwist
}) {
    const [selected, setSelected] = useState();

    const updateSelected = useCallback((next) => {
        if (typeof selected === 'number') {
            if (selected === next) {
                if (selected < 4) {
                    onFlip(selected);
                } else {
                    onTwist(selected - 4);
                }
                setSelected(undefined);
            } else {
                if (selected < 4 && next < 4) {
                    onSwapEdges(selected, next);
                    setSelected(undefined);
                } else if (selected >= 4 && next >= 4) {
                    onSwapCorners(selected - 4, next - 4);
                    setSelected(undefined);
                }
            }
        } else {
            setSelected(next);
        }
    }, [selected]);

    // TODO: validation

    return (
        <svg width="400" height="400" viewBox="15 15 84 84" className="ll">
            <Edge selected={selected === 0} onClick={() => updateSelected(0)}
                x="50" y="35" data={edges[0]} rotate="-90"/>
            <Edge selected={selected === 1} onClick={() => updateSelected(1)}
                x="65" y="50" data={edges[1]} rotate="0" />
            <Edge selected={selected === 2} onClick={() => updateSelected(2)}
                x="50" y="65" data={edges[2]} rotate="90" />
            <Edge selected={selected === 3} onClick={() => updateSelected(3)}
                x="35" y="50" data={edges[3]} rotate="180" />
            <Corner selected={selected === 4} onClick={() => updateSelected(4)}
                x="35" y="35" rotate="0" data={corners[0]} />
            <Corner selected={selected === 5} onClick={() => updateSelected(5)}
                x="65" y="35" rotate="90" data={corners[1]} />
            <Corner selected={selected === 6} onClick={() => updateSelected(6)}
                x="65" y="65" rotate="180" data={corners[2]} />
            <Corner selected={selected === 7} onClick={() => updateSelected(7)}
                x="35" y="65" rotate="270" data={corners[3]} />
            <rect x="50" y="50" width="12" height="12" fill={yellow} />
        </svg>
    );
}


function getSelectedProps(selected) {
    if (selected) {
        return {
            stroke: '#CCC',
            strokeWidth: 1,
            strokeDasharray: '4,3',
        };
    } else {
        return {};
    }
}

function Corner(props) {
    const colors = rotate([
        yellow,
        getSideColor(props.data[0]),
        getSideColor(props.data[0] + 1)
    ], -props.data[1]);

    const selectedProps = getSelectedProps(props.selected);

    return (
        <g
            transform={`
                translate(${props.x},${props.y})
                rotate(${props.rotate || 0} 6 6)
            `}
            onClick={props.onClick}
        >
            <rect
                width="12"
                height="12"
                fill={colors[0]}
                {...selectedProps}
            />
            <rect
                y="-15"
                width="12"
                height="12"
                fill={colors[2]}
                {...selectedProps}
            />
            <rect
                x="-15"
                width="12"
                height="12"
                fill={colors[1]}
                {...selectedProps}
            />
        </g>
    );
}

function Edge(props) {
    const color = getSideColor(props.data[0] + 1);
    const selectedProps = getSelectedProps(props.selected);
    return (
        <g
            transform={`
                translate(${props.x},${props.y})
                rotate(${props.rotate || 0} 6 6)
            `}
            onClick={props.onClick}
        >
            <rect
                width="12"
                height="12"
                fill={props.data[1] ? color : yellow}
                {...selectedProps}
            />
            <rect
                x="15"
                width="12"
                height="12"
                fill={!props.data[1] ? color : yellow}
                {...selectedProps}
            />
        </g>
    );
}
