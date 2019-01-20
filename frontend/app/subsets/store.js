import React, { createContext, useContext, useState } from 'react';
import LL from './ll';

const ctx = createContext();

export const useCases = () => {
    return useContext(ctx);
}

export const CaseStore = ({  children }) => {
    const [cases, setCases] = useState([]);
    const [subset, setSubset] = useState([]);

    return <ctx.Provider
        value={{
            cases, setCases,
            subset, setSubset,
        }}
        children={children}
    />
}
