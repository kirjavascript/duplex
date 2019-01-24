import React, { Fragment } from 'react';
import { useSolver } from '#app/solver';
import { useAlgs, AlgStore } from './store';

function Checkbox({ checked, onChange, name }) {
    return (
        <div className="checkbox">
            {name}
            <span className="link" onClick={onChange}>
                {checked?'yes':'no'}
            </span>
        </div>
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
        deleteAlg,
        parseError,
        setParseError,
    } = useAlgs();

    const {
        loadAlgs,
    } = useSolver();

    return (
        <Fragment>
            TODO: storage/csv
            <button
                type="button"
                onClick={() => {
                    setParseError();
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
            {parseError && (
                <pre style={{color:'red'}}>
                    {parseError}
                </pre>
            )}
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
                    <Checkbox
                        checked={alg.mirror}
                        onChange={() => { toggleMirror(i); }}
                        name="mirror"
                    />
                    <Checkbox
                        checked={alg.invert}
                        onChange={() => { toggleInvert(i); }}
                        name="invert"
                    />
                    <button
                        type="button"
                        onClick={() => { deleteAlg(i); }}
                    >
                        delete
                    </button>
                </div>
            ))}
        </Fragment>
    );
}
