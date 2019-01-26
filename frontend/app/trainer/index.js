import React, { Fragment, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useSolver } from '#app/solver';
import { useTrainer } from './store';

export default function Trainer() {

    const { trainerCase } = useTrainer();
    const { loadTrainerCase } = useSolver();

    return (
        <Fragment>
            <pre>
                {JSON.stringify(trainerCase,0,4)}
            </pre>
            <Link to="/subsets">
                choose a subset
            </Link>
            <span className="grey">
                setup:
            </span>
            <button onClick={loadTrainerCase}>
                loadnew
            </button>
        </Fragment>
    );
}
