import React, { createContext, useContext, useState } from 'react';

const ctx = createContext();

export const useSolutions = () => {
    return useContext(ctx);
}

export const SolutionStore = ({  children }) => {
    const [solutions, setSolutions] = useState([]);
    const [solving, setSolving] = useState(true);

    return <ctx.Provider
        value={{
            solutions, setSolutions,
            solving, setSolving,
        }}
        children={children}
    />
}
