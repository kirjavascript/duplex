import React, { createContext, useContext, useState } from 'react';
import useLocalState from '#app/localstate';
import LL from './ll';
import { blank } from './subset-list';

const ctx = createContext();

export const useCases = () => {
    return useContext(ctx);
}

export const CaseStore = ({  children }) => {
    const [cases, setCases] = useState([]);
    const [solutions, setSolutions] = useState([]);
    const [solving, setSolving] = useState(true);

    const [ll, setLL] = useLocalState('LL', blank);
    const [sticker, setSticker] = useLocalState('sticker', 'X');
    const [sort, setSort] = useLocalState('sort', 'canonical');

    return <ctx.Provider
        value={{
            cases, setCases,
            solutions, setSolutions,
            solving, setSolving,
            ll, setLL,
            sticker, setSticker,
            sort, setSort,
        }}
        children={children}
    />
}
