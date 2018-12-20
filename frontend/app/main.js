import React, { useState, useEffect, useCallback, Fragment } from 'react';
import { render } from 'react-dom';

function App() {

    return (
        <Fragment>
            hello
            <svg width="200" height="200">

            </svg>
        </Fragment>
    );
}

render((
    <App />
), document.body.appendChild(document.createElement('div')));
