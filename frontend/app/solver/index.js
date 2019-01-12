
import React, { useEffect } from 'react';
import { useAlgs } from '#app/algs/store';
import { useCases } from '#app/subsets/store';
import { useSolutions } from './store';
import Worker from './worker';

export default function Solver() {
    const { algs } = useAlgs();
    const { cases, setCases } = useCases();
    const { solutions, setSolutions } = useSolutions();

    useEffect(() => {
        const worker = new Worker();

        worker.addEventListener('message', ({ data: { action, payload } }) => {
            if (action === 'INIT') {
                worker.postMessage({
                    action: 'LOAD_ALGS',
                    payload: algs,
                });
            } else if (action === 'CASES') {
                setCases(payload);
            } else if (action === 'SOLUTIONS') {
                setSolutions(payload);
            }
        });

        // worker.postMessage({
        //     action: 'EXPLORE_SOLVE',
        //     payload: transform,
        // });
    }, []);

    return false;
}
