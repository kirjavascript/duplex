import React, { Fragment } from 'react';
import { useCases } from './store';

export default function Subsets() {
    const { cases } = useCases();

    return (
        <Fragment>
            <pre>{JSON.stringify(cases, null, 4)}</pre>
        </Fragment>
    );
}
