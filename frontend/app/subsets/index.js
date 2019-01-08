import React, { Fragment } from 'react';
import { useCases } from './store';

const faces = {
    U: '#FFFF5A',
    L: '#FA2222',
    F: '#90EE90',
    B: 'steelblue',
    R: '#FAA222',
};


// TODO: AUF, change colour

function Edge(props) {
    return (
        <g
            transform={`
                translate(${props.x},${props.y})
                rotate(${props.rotate || 0} 6 6)
            `}
        >
            <rect width="12" height="12" fill={faces[props.stickers[1]]} />
            <rect x="15" width="12" height="12" fill={faces[props.stickers[0]]} />
        </g>
    );
}

function Corner(props) {
    return (
        <g
            transform={`
                translate(${props.x},${props.y})
                rotate(${props.rotate || 0} 6 6)
            `}
        >
            <rect width="12" height="12" fill={faces[props.stickers[0]]} />
            <rect y="-15" width="12" height="12" fill={faces[props.stickers[2]]} />
            <rect x="-15" width="12" height="12" fill={faces[props.stickers[1]]} />
        </g>
    );
}

export default function Subsets() {
    const { cases } = useCases();

    return (
        <Fragment>
            {cases.length} cases <br />
            {cases.slice(0, 100).map((case_, i) => {

                return <svg
                    width="100"
                    height="100"
                    viewBox="0 0 99 99"
                    key={case_.index}
                >
                    <Edge stickers={case_.edges[0]} x={30} y={0} rotate={90} />
                    <Edge stickers={case_.edges[1]} x={60} y={30} rotate={180} />
                    <Edge stickers={case_.edges[2]} x={30} y={60} rotate={270} />
                    <Edge stickers={case_.edges[3]} x={0} y={30} />
                    <Corner stickers={case_.corners[0]} x={15} y={15} />
                    <Corner stickers={case_.corners[1]} x={45} y={15} rotate={90} />
                    <Corner stickers={case_.corners[2]} x={45} y={45} rotate={180} />
                    <Corner stickers={case_.corners[3]} x={15} y={45} rotate={270} />
                    <rect x="30" y="30" width="12" height="12" fill={faces.U} />
                </svg>
            })}
        </Fragment>
    );
}
