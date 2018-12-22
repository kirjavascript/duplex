import React, { Fragment, useEffect } from 'react';
// import {  } from './index';

export default function Explore() {

    return (
        <Fragment>
            <LL
                edges={[[0, 0], [1, 0], [2, 0], [3, 0]]}
                corners={[[0, 0], [1, 0], [2, 0], [3, 0]]}
            />
        </Fragment>
    );
}

const [yellow, red, green, blue, orange] = [
    '#FFFF5A', '#FA2222', '#90EE90', 'steelblue', '#FAA222',
];

const getSideColor = (index) => (
    [orange, blue, red, green][index%4]
);

function LL({ edges, corners }) {
    const swapSelected = undefined;

    return (
        <svg width="400" height="400" viewBox="15 15 84 84">
            <Edge x="50" y="35" data={edges[0]} rotate="-90"/>
            <Edge x="65" y="50" data={edges[1]} rotate="0" />
            <Edge x="50" y="65" data={edges[2]} rotate="90" />
            <Edge x="35" y="50" data={edges[3]} rotate="180" />
            <rect x="50" y="50" width="12" height="12" fill={yellow} />
            <Corner x="35" y="35" rotate="0" data={corners[0]} />
            <Corner x="65" y="35" rotate="90" data={corners[1]} />
            <Corner x="65" y="65" rotate="180" data={corners[2]} />
            <Corner x="35" y="65" rotate="270" data={corners[3]} />
        </svg>
    );
}

function rotate(arr, n) {
    return arr.slice(n, arr.length).concat(arr.slice(0, n));
}

function Corner(props) {
    const colors = rotate([
        yellow,
        getSideColor(props.data[0]),
        getSideColor(props.data[0] + 1)
    ], -props.data[1]);

    return (
        <g
            transform={`
                translate(${props.x},${props.y})
                rotate(${props.rotate || 0} 6 6)
            `}
        >
            <rect
                width="12"
                height="12"
                fill={colors[0]}
            />
            <rect
                y="-15"
                width="12"
                height="12"
                fill={colors[2]}
            />
            <rect
                x="-15"
                width="12"
                height="12"
                fill={colors[1]}
            />
        </g>
    );
}

function Edge(props) {
    const color = getSideColor(props.data[0] + 1);
    return (
        <g
            transform={`
                translate(${props.x},${props.y})
                rotate(${props.rotate || 0} 6 6)
            `}
        >
            <rect
                width="12"
                height="12"
                fill={props.data[1] ? color : yellow}
            />
            <rect
                x="15"
                width="12"
                height="12"
                fill={!props.data[1] ? color : yellow}
            />
        </g>
    );
}
