const faces = {
    U: '#FFFF5A',
    L: '#FA2222',
    F: '#90EE90',
    B: 'steelblue',
    R: '#FAA222',
    X: '#C0c0c0',
};

function Edge({ x, y, rotate, stickers, index }) {
    return (
        <g
            transform={`
                translate(${x},${y})
                rotate(${rotate || 0} 6 6)
            `}
        >
            <rect type="edges" perm={index} orient={1}
                width="12" height="12" fill={faces[stickers[1]]} />
            <rect type="edges" perm={index} orient={0}
                x="15" width="12" height="12" fill={faces[stickers[0]]} />
        </g>
    );
}

function Corner({ x, y, rotate, stickers, index }) {
    return (
        <g
            transform={`
                translate(${x},${y})
                rotate(${rotate || 0} 6 6)
            `}
        >
            <rect type="corners" perm={index} orient={0}
                width="12" height="12" fill={faces[stickers[0]]} />
            <rect type="corners" perm={index} orient={2}
                y="-15" width="12" height="12" fill={faces[stickers[2]]} />
            <rect type="corners" perm={index} orient={1}
                x="-15" width="12" height="12" fill={faces[stickers[1]]} />
        </g>
    );
}

export default function LL({
    case_,
    width = 120,
    height = 120,
    onClick,
}) {

    const { corners, edges } = case_;

    return (
        <svg
            width={width}
            height={height}
            viewBox="0 0 72 72"
            className="ll"
            onClick={onClick && ((e) => {
                const type = e.target.getAttribute('type');
                const perm = +e.target.getAttribute('perm');
                const orient = +e.target.getAttribute('orient');
                type && onClick({ type, perm, orient });
            })}
        >
            <Edge index={0} stickers={edges[0]} x={30} y={0} rotate={90} />
            <Edge index={1} stickers={edges[1]} x={60} y={30} rotate={180} />
            <Edge index={2} stickers={edges[2]} x={30} y={60} rotate={270} />
            <Edge index={3} stickers={edges[3]} x={0} y={30} />
            <Corner index={0} stickers={corners[0]} x={15} y={15} />
            <Corner index={1} stickers={corners[1]} x={45} y={15} rotate={90} />
            <Corner index={2} stickers={corners[2]} x={45} y={45} rotate={180} />
            <Corner index={3} stickers={corners[3]} x={15} y={45} rotate={270} />
            <rect x="30" y="30" width="12" height="12" fill={faces.U} />
        </svg>
    );
}

LL.faces = faces;
