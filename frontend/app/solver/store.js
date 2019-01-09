import React, { createContext, useContext, useState } from 'react';

const ctx = createContext();

export const useSolutions = () => {
    return useContext(ctx);
}

export const SolutionStore = ({  children }) => {
    const [solutions, setSolutionsReal] = useState([]);
    const [length, setLength] = useState(0);

    const setSolutions = (solutions) => {
        setLength(Object.keys(solutions).length);
        setSolutionsReal(solutions);
    };

    return <ctx.Provider
        value={{
            solutions, setSolutions, length,
        }}
        children={children}
    />
}
