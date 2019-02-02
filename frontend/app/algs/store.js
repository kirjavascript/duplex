import React, { createContext, useState, useCallback, useContext } from 'react';
import produce from 'immer';
import useLocalState from '#app/localstate';
import defaultList from './default-list';

const ctx = createContext();

export const AlgStore = ({  children }) => {
    const [algs, setAlgs] = useLocalState('algs', defaultList);
    const [parseError, setParseError] = useState();

    const setAlgsMut = (cb) => setAlgs(produce(cb));

    const addAlg = useCallback((e) => {
        setAlgsMut(state => [...state, {
            moves: '',
            name: '',
            mirror: 'FB',
            invert: true,
        }]);
    }, []);

    const updateMoves = useCallback((index, str) => {
        setAlgsMut(state => {
            state[index].moves = str;
        });
    }, []);

    const updateName = useCallback((index, str) => {
        setAlgsMut(state => {
            state[index].name = str;
        });
    }, []);

    const toggleMirror = useCallback((index) => {
        setAlgsMut(state => {
            const { mirror } = state[index];
            if (!mirror) {
                state[index].mirror = 'FB';
            } else if (mirror === 'FB') {
                state[index].mirror = 'LR';
            } else {
                state[index].mirror = null;
            }
        });
    }, []);


    const toggleInvert = useCallback((index) => {
        setAlgsMut(state => {
            state[index].invert = !state[index].invert;
        });
    }, []);

    const deleteAlg = useCallback((index) => {
        setAlgsMut(state => {
            state.splice(index, 1);
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
            parseError,
            setParseError,
        }}
        children={children}
    />
}

export const useAlgs = () => {
    return useContext(ctx);
}
