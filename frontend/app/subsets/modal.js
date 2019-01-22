import React, { Component, useRef, useEffect } from 'react';
import { createPortal } from 'react-dom';
import LL from './ll';

export default function Modal({ case_, solutions }) {
    const container = useRef();

    useEffect(() => {
        container.current = document.body.appendChild(document.createElement('div'));

        return () => {
            document.body.removeChild(container.current);
        };
    }, []);

    return createPortal((
        <div className="modal">
            <div className="modal-box">
                <LL case_={case_} />
                {children}
            </div>
        </div>
    ), container.current);
}
