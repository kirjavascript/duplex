import React, { Fragment, useEffect } from 'react';
// import {  } from './index';

export default function Explore() {

    return (
        <Fragment>
            <LL />
            asd
        </Fragment>
    );
}

function LL() {
    return (
        <svg width="500" height="500" viewBox="0 0 500 500">
            <Sticker x="10" y="10" />
        </svg>
    );
}

function Sticker(props) {
    return (
        <rect
            width="10"
            height="10"
            {...props}
        />
    );
}
