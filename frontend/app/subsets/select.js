import React, { Fragment, useState } from 'react';
import produce from 'immer';
import { useSolver } from '#app/solver';
import LL from './ll';
import { useCases } from './store';

function Picker() {
    const { sticker, setSticker } = useCases();
    const [faces] = useState(() => {
        return Object.keys(LL.faces)
            .map(key => ({ key, value: LL.faces[key] }));
    });
    return (
        <div className="picker">
            <div
                className="sticker"
                style={{
                    backgroundColor: LL.faces[sticker],
                    border: '5px solid white',
                    marginRight: '5px',
                }}
            />
            {faces.map((face, i) => (
                <div
                    key={i}
                    className="sticker"
                    style={{
                        backgroundColor: face.value,
                    }}
                    onClick={() => {
                        setSticker(face.key);
                    }}
                />
            ))}
        </div>
    );
}

export default function Select() {

    const { loadSubset } = useSolver();
    const { ll, setLL, sticker } = useCases();

    return (
        <div className="select">
            <LL
                case_={ll}
                width={280}
                height={280}
                onClick={({ type, perm, orient }) => {
                    const newPos = produce(ll, ll => {
                        ll[type][perm][orient] = sticker;
                    });
                    setLL(newPos);
                    loadSubset({ index: '0', ...newPos });
                }}
            />
            <Picker />
        </div>
    );
}
