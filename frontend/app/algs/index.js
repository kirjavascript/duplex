import React, { Fragment } from 'react';
import { useSolver } from '#app/solver';
import { useCases } from '#app/subsets/store';
import { useAlgs, AlgStore } from './store';

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

    const { loadAlgs } = useSolver();

    const { solving } = useCases();

    return (
        <div className="algs">
            <div className="options">
                {solving ? (
                    <button type="button">solving...</button>
                ) : (
                    <button
                        type="button"
                        onClick={() => {
                            setParseError();
                            loadAlgs(algs);
                        }}
                    >
                        find solutions
                    </button>
                )}
                <button
                    type="button"
                    onClick={addAlg}
                >
                    add new
                </button>
                {parseError && (
                    <pre className="parse-error">
                        {parseError}
                    </pre>
                )}
            </div>
            {algs.map((alg, i) => (
                <div key={i} className="alg">
                    <input
                        type="text"
                        value={alg.name}
                        placeholder="name"
                        onChange={(e) => { updateName(i, e.target.value); }}
                    />
                    <input
                        className="inner-text"
                        type="text"
                        value={alg.moves}
                        placeholder="moves"
                        onChange={(e) => { updateMoves(i, e.target.value); }}
                    />
                    <div className="checkbox">
                        mirror
                        <span className="link" onClick={() => {
                            toggleMirror(i);
                        }}>
                            {alg.mirror || 'no'}
                        </span>
                    </div>

                    <div className="checkbox">
                        invert
                        <span className="link" onClick={() => {
                            toggleInvert(i);
                        }}>
                            {alg.invert?'yes':'no'}
                        </span>
                    </div>
                    <button
                        type="button"
                        onClick={() => { deleteAlg(i); }}
                    >
                        delete
                    </button>
                </div>
            ))}
        </div>
    );
}
