import React, { Fragment, useEffect } from 'react';
import Algs, { AlgStore, useAlgs } from '#app/algs';
import { startWorker, loadAlgs } from './index';

export default function Solver() {

    const { algs } = useAlgs();
    useEffect(() => {
        startWorker(() => {
            loadAlgs(algs);
        });
    }, []);

    return false;
}
