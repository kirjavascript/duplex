import React, { createContext, useContext, useState, useCallback } from 'react';
import useLocalState from '#app/localstate';
import produce from 'immer';
import LL from './ll';
import { blank } from './subset-list';

const ctx = createContext();

export const useCases = () => {
    return useContext(ctx);
}

export const CaseStore = ({  children }) => {
    const [cases, _setCases] = useState([]);
    const [solutions, setSolutions] = useState([]);
    const [solving, setSolving] = useState(true);

    const [ll, setLL] = useLocalState('LL', blank);
    const [sticker, setSticker] = useLocalState('sticker', 'U');
    const [sort, setSort] = useLocalState('sort', 'canonical');

    const setCases = useCallback((cases) => {
        _setCases(cases.map((obj) => {
            return { trainer: true, ...obj };
        }));
    }, []);

    const toggleTrainer = useCallback((index) => {
        const newCases = produce(cases, draft => {
            draft[index].trainer = !draft[index].trainer;
        });
        setCases(newCases);
    }, [cases]);

    const trainerAll = useCallback((value = true) => {
        const newCases = produce(cases, draft => {
            draft.forEach(case_ => { case_.trainer = value });
        });
        setCases(newCases);
    }, [cases]);


    return <ctx.Provider
        value={{
            cases, setCases,
            solutions, setSolutions,
            solving, setSolving,
            ll, setLL,
            sticker, setSticker,
            sort, setSort,
            toggleTrainer,
            trainerAll,
        }}
        children={children}
    />
}
