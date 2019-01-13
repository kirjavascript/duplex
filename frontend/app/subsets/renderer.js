import React, { Fragment, useRef } from 'react';
import { useWindowSize, useElementYPos } from './render-hooks';

const boxWidth = 320;
const boxHeight = 320;
const leeway = boxHeight;
const margin = 5;

export default function Renderer({ caseList, children }) {

    const { width, height } = useWindowSize();
    const { ref, posY } = useElementYPos();
    const widthTrim = (width - (2 * margin));
    const { length: quantity } = caseList;
    const rawWidth = boxHeight + margin;
    const rawHeight = boxWidth + margin;
    const columns = Math.max(Math.floor(widthTrim / rawWidth), 1);
    const rows = Math.ceil(quantity/columns);
    const wrapperHeight = rows * rawHeight;
    const overflow = widthTrim <= rawWidth ? 0 : ((widthTrim % rawWidth) / 2);

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
                            border: '1px solid black',
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
