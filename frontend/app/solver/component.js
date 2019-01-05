import React, { useEffect } from 'react';
import { useAlgs } from '#app/algs/store';
import { useCases } from '#app/subsets/store';
import { startWorker, loadAlgs } from './index';

export default function Solver() {
    const { algs } = useAlgs();
    const { cases, setCases } = useCases();

    useEffect(() => {
        startWorker({
            onload: () => {
                loadAlgs(algs);
            },
            cases: (payload) => {
                setCases(payload);
            },
        });
    }, []);

    return false;
}
