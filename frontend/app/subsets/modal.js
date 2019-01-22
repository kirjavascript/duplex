import React, { Component, useRef, useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import LL from './ll';

export default function Modal({ case_, solutions }) {
    const container = useRef();
    const [display, setDisplay] = useState(false);

    useEffect(() => {
        container.current = document.body.appendChild(document.createElement('div'));
        setDisplay(true);
        return () => {
            document.body.removeChild(container.current);
        };
    }, []);

    return display && createPortal((
        <div className="modal">
            <div className="modal-box">
                <LL case_={case_} />
            </div>
        </div>
    ), container.current);
}
