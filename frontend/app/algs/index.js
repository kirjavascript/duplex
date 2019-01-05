import React, { Fragment } from 'react';
import { loadAlgs } from '#app/solver';
import { useAlgs, AlgStore } from './store';

function Checkbox({ checked, onChange }) {
    return (
        <span className="link" onClick={onChange}>
            {checked?'yes':'no'}
        </span>
    );
}

export default function Algs() {
    const {
        algs,
        addAlg,
        updateMoves,
        updateName,
        toggleMirror,
        toggleInvert,
        deleteAlg
    } = useAlgs();

    return (
        <Fragment>
            TODO: storage/csv
            {algs.map((alg, i) => (
                <div key={i} className="alg">
                    <input
                        type="text"
                        value={alg.name}
                        placeholder="name"
                        onChange={(e) => { updateName(i, e.target.value); }}
                    />
                    <input
                        className="moves"
                        type="text"
                        value={alg.moves}
                        placeholder="moves"
                        onChange={(e) => { updateMoves(i, e.target.value); }}
                    />
                    mirror
                    <Checkbox
                        checked={alg.mirror}
                        onChange={() => { toggleMirror(i); }}
                    />
                    invert
                    <Checkbox
                        checked={alg.invert}
                        onChange={() => { toggleInvert(i); }}
                    />
                    <button
                        type="button"
                        onClick={() => { deleteAlg(i); }}
                    >
                        delete
                    </button>
                </div>
            ))}
            <button
                type="button"
                onClick={() => {
                    loadAlgs(algs);
                }}
            >
                reload algs
            </button>
            <button
                type="button"
                onClick={addAlg}
            >
                add
            </button>
        </Fragment>
    );
}
