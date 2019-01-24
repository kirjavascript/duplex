import React, { Component, Fragment, useRef, useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import LL from './ll';

export default function Modal({ show = false, ...props }) {
    return show && <ModalDOM {...props} />
}

function ModalDOM({ case_, solutions, children }) {
    const container = useRef();
    const [display, setDisplay] = useState(false);

    useEffect(() => {
        container.current = document.body.appendChild(document.createElement('div'));
        document.body.style.overflow = 'hidden';
        setDisplay(true);
        return () => {
            document.body.removeChild(container.current);
            document.body.style.overflow = 'initial';
        };
    }, []);

    return display && createPortal((
        <div className="modal">
            <div className="modal-box">
                {children()}
            </div>
        </div>
    ), container.current);
}
