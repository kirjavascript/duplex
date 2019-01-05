import React, { createContext, useState, useCallback, useContext } from 'react';
import defaultList from './default-list';

const ctx = createContext();

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
