import React, { createContext, useContext, useState } from 'react';
import useLocalState from '#app/localstate';
import LL from './ll';
import { solved } from './subset-list';

const ctx = createContext();

export const useCases = () => {
    return useContext(ctx);
}

export const CaseStore = ({  children }) => {
    const [cases, setCases] = useLocalState('cases', []);
    const [subset, setSubset] = useLocalState('subset', []);
    const [ll, setLL] = useLocalState('LL', solved);
    const [sticker, setSticker] = useLocalState('sticker', 'X');
    const [select, setSelect] = useLocalState('select', 'transform');

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
