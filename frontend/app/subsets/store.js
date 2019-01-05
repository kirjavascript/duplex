import React, { createContext, useContext, useState } from 'react';

const ctx = createContext();

export const useCases = () => {
    return useContext(ctx);
}

export const CaseStore = ({  children }) => {
    const [cases, setCases] = useState([]);

    return <ctx.Provider
        value={{
            cases, setCases,
        }}
        children={children}
    />
}
