import React, { createContext, useState, useCallback, useContext, Fragment } from 'react';
import defaultList from './default-list';

// state

export const ctx = createContext();

export const AlgStore = ({  children }) => {
    const [algs, setAlgs] = useState(defaultList);

    const addAlg = useCallback((obj) => {
        setAlgs(state => [...state, {
            moves: '',
            name: '',
            mirror: true,
            invert: true,
        }]);
    }, []);

    const updateMoves = useCallback((index, str) => {
        setAlgs(state => {
            state[index].moves = str;
            return state;
        });
    }, []);

    const updateName = useCallback((index, str) => {
        setAlgs(state => {
            state[index].name = str;
            return state;
        });
    }, []);

    const toggleMirror = useCallback((index) => {
        setAlgs(state => {
            state[index].mirror = !state[index].mirror;
            return state;
        });
    }, []);


    const toggleInvert = useCallback((index) => {
        setAlgs(state => {
            state[index].invert = !state[index].invert;
            return state;
        });
    }, []);

    const deleteAlg = useCallback((index) => {
        setAlgs(state => {
            state.splice(index, 1);
            return state;
        });
    }, []);

    return <ctx.Provider
        value={{
            algs,
            addAlg,
            deleteAlg,
            updateMoves,
            updateName,
            toggleMirror,
            toggleInvert,
        }}
        children={children}
    />
}

export const useAlgs = () => {
    return useContext(ctx);
}

// ui

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
            TODO: storage/styles
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
                onClick={addAlg}
                className="fullwidth"
            >
                add
            </button>
        </Fragment>
    );
}
