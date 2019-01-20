import React, { Fragment, useState } from 'react';
import { useSolver } from '#app/solver';
import produce from 'immer';
import LL from './ll';

// useLL

function Picker({ sticker, setSticker }) {
    const [faces] = useState(() => {
        return Object.keys(LL.faces)
            .map(key => ({ key, value: LL.faces[key] }));
    });
    return (
        <div className="picker">
            {faces.map((face, i) => (
                <div
                    key={i}
                    className="sticker"
                    style={{backgroundColor: face.value}}
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

    const [ll, setLL] = useState(LL.default);
    const [sticker, setSticker] = useState('X');

    const reset = () => {
        setLL(LL.default);
        loadSubset({ index: '0', ...LL.default });
    };

    return (
        <Fragment>
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
            <Picker
                sticker={sticker}
                setSticker={setSticker}
            />
            <button onClick={reset}>
                reset
            </button>
        </Fragment>
    );
}
