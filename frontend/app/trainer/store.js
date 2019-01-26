import React, { createContext, useContext, useState } from 'react';
import { solved } from '#app/subsets/subset-list';

const ctx = createContext();

export const useTrainer = () => {
    return useContext(ctx);
}

export const TrainerStore = ({  children }) => {
    const [trainerCase, setTrainerCase] = useState(solved);

    return <ctx.Provider
        value={{
            trainerCase, setTrainerCase,
        }}
        children={children}
    />
}
