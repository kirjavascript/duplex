import React, { createContext, useContext, useState } from 'react';
import LL from './ll';
import { solved } from './subset-list';

const ctx = createContext();

export const useCases = () => {
    return useContext(ctx);
}

export const CaseStore = ({  children }) => {
    const [cases, setCases] = useState([]);
    const [subset, setSubset] = useState([]);
    const [ll, setLL] = useState(solved);
    const [sticker, setSticker] = useState('X');
    const [select, setSelect] = useState('transform');

    return <ctx.Provider
        value={{
            cases, setCases,
            subset, setSubset,
            ll, setLL,
            sticker, setSticker,
            select, setSelect,
        }}
        children={children}
    />
}
