import { useState, useEffect, useRef } from 'react';

export function useWindowSize() {
    const [size, setSize] = useState({
        width: window.innerWidth,
        height: window.innerHeight,
    });
    useEffect(() => {
        const handleResize = () => setSize({
            width: window.innerWidth,
            height: window.innerHeight,
        });
        window.addEventListener('resize', handleResize);
        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, []);
    return size;
}

export function useElementYPos() {
    const ref = useRef(null);
    const [posY, setPosY] = useState(0);

    useEffect(() => {
        const handleMove = () => setPosY(
            ref.current ? ref.current.getBoundingClientRect().y : 0
        );
        handleMove();
        const asyncHandleMove = () => {
            requestAnimationFrame(handleMove);
        };
        window.addEventListener('scroll', asyncHandleMove);
        return () => {
            window.removeEventListener('scroll', asyncHandleMove);
        };
    }, []);

    return { ref, posY };
}
