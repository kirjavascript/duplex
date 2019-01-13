import React, { Fragment, useState, useEffect, useRef } from 'react';

function useWindowWidth() {
    const [width, setWidth] = useState(window.innerWidth);
    useEffect(() => {
        const handleResize = () => setWidth(window.innerWidth);
        window.addEventListener('resize', handleResize);
        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, []);
    return width;
}

const boxWidth = 320;
const boxHeight = 320;
const margin = 5;

export default function Renderer({ caseList, children }) {

    const width = useWindowWidth();
    const widthTrim = (width - (2 * margin));
    const { scrollY } = window;
    const { length: quantity } = caseList;
    const wrapper = useRef(null);

    const columns = Math.floor(widthTrim / (boxWidth + margin));
    const rows = Math.ceil(quantity/columns);
    const height = rows * (boxHeight + margin);

    return (
        <div
            style={{
                margin: `0px ${margin}px`,
                height: `${height}px`,
            }}
        >
        </div>
    );
}
