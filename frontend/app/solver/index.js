import React, {
    createContext,
    useContext,
    useEffect,
    useState,
    useCallback,
    useRef,
} from 'react';

import { useAlgs } from '#app/algs/store';
import { useCases } from '#app/subsets/store';
import Worker from './worker';

const ctx = createContext();

export function useSolver() {
    const { workerRef: { current: worker } } = useContext(ctx);

    function loadAlgs(algs) {
        worker.postMessage({
            action: 'LOAD_ALGS',
            payload: algs,
        });
    }

    function loadSubset({ ll, sort }) {
        worker.postMessage({
            action: 'LOAD_SUBSET',
            payload: { ll, sort },
        });
    }

    return {
        loadAlgs, loadSubset, worker,
    };
}

export default function Solver({ children }) {
    const { algs, setParseError } = useAlgs();
    const {
        cases,
        setCases,
        solutions,
        setSolutions,
        setSolving,
        ll,
        sort,
    } = useCases();
    const workerRef = useRef();

    useEffect(() => {
        const worker = new Worker();

        worker.addEventListener('message', ({ data: { action, payload } }) => {
            if (action === 'INIT') {
                worker.postMessage({
                    action: 'LOAD_ALGS',
                    payload: algs,
                });
            } else if (action === 'PARSE_ERROR') {
                setParseError(payload);
            } else if (action === 'START_SOLVE') {
                setSolving(true);
            } else if (action === 'END_SOLVE') {
                setSolving(false);
                worker.postMessage({
                    action: 'LOAD_SUBSET',
                    payload: {
                        ll, sort,
                    },
                })
            } else if (action === 'SOLUTIONS') {
                setSolutions(payload);
            } else if (action === 'CASES') {
                setCases(payload);
            }
        });

        workerRef.current = worker;

    }, []);

    return <ctx.Provider
        value={{
            workerRef,
        }}
        children={children}
    />;
}
