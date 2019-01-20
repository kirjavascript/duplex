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
import { useSolutions } from './store';
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

    function loadSubset(subset) {
        worker.postMessage({
            action: 'LOAD_SUBSET',
            payload: subset,
        });
    }

    return {
        loadAlgs, loadSubset,
    };
}

export default function Solver({ children }) {
    const { algs, setParseError } = useAlgs();
    const { cases, setCases, setSubset } = useCases();
    const { solutions, setSolutions } = useSolutions();
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
            } else if (action === 'CASES') {
                setCases(payload);
            } else if (action === 'SOLUTIONS') {
                setSolutions(payload);
            } else if (action === 'SUBSET') {
                setSubset(payload);
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
