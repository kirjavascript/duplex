import React, { Fragment, useRef } from 'react';
import { useWindowSize, useElementYPos } from './render-hooks';

const boxWidth = 300;
const boxHeight = 300;
const leeway = boxHeight;
const margin = 10;

export default function Renderer({ caseList, children }) {

    const { width, height } = useWindowSize();
    const { ref, posY } = useElementYPos();
    const { length: quantity } = caseList;
    const rawWidth = boxWidth + margin;
    const rawHeight = boxHeight + margin;
    const columns = Math.max(Math.floor(width / rawWidth), 1);
    const rows = Math.ceil(quantity/columns);
    const wrapperHeight = rows * rawHeight;
    const overflow = width <= rawWidth ? 0 : ((width % rawWidth) / 2);

    let keyIndex = 0;

    return (
        <div
            style={{
                margin: `0px ${overflow}px`,
                height: `${wrapperHeight}px`,
                position: 'relative',
            }}
            ref={ref}
        >
            {caseList.map((obj, i) => {
                const x = i % columns;
                const y = Math.floor(i / columns);
                const left = x * rawWidth;
                const top = y * rawHeight;
                const bottom = top + boxHeight;
                const onScreen = bottom + posY +leeway > 0
                    && (top + posY) - leeway < height;
                if (onScreen) {
                    keyIndex += 1;
                }
                return onScreen ? (
                    <div
                        key={keyIndex}
                        style={{
                            left,
                            top,
                            width: boxWidth,
                            height: boxHeight,
                            position: 'absolute',
                        }}
                    >
                        {children(obj)}
                    </div>
                ) : false;
            })}
        </div>
    );
}
