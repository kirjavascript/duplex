import React, { Fragment, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import normalize from 'cube-notation-normalizer';
import { useSolver } from '#app/solver';
import LL from '#app/subsets/ll';
import { useWindowSize } from '#app/subsets/render-hooks';
import { Moves } from '#app/subsets/case';
import { useCases } from '#app/subsets/store';
import { blank } from '#app/subsets/subset-list';

const auf = ['', 'U', 'U2', 'U\''];

export function solutionToString(solution) {
    return solution.alg.map(part => {
        if (typeof part === 'number') {
            return auf[part];
        } else {
            return part.moves;
        }
    }).join(' ');
}

function getSetup(solutions) {
    const movesList = solutions.map(solution => {
        const moves = normalize(solutionToString(solution), { invert: true });
        return {
            moves,
            length: moves.split(' ').length,
        };
    }).sort((a, b) => {
        return a.length - b.length;
    });

    return movesList[0].moves;
}

export default function Trainer() {

    const { width } = useWindowSize();
    const LLSize = Math.min(400, width - 40);

    const [trainerCase, setTrainerCase] = useState({ case: blank });
    const { cases, solutions } = useCases();

    const loadTrainerCase = () => {
        const casesTrainer = cases.filter(d => d.trainer);
        if (casesTrainer.length) {
            const index = Math.floor(Math.random()*casesTrainer.length);
            setTrainerCase(casesTrainer[index]);
        }
    };

    const solutionList = do {
        if (trainerCase.sIds) {
            trainerCase.sIds.map(i => solutions[i]);
        } else {
            [];
        }
    };

    useEffect(() => {
        loadTrainerCase();
        const getSpace = (e) => {
            if (e.keyCode === 32) {
                loadTrainerCase();
            }
        };
        const eatSpace = (e) => {
            if (e.keyCode === 32) {
                e.preventDefault();
            }
        };
        document.addEventListener('keyup', getSpace)
        document.addEventListener('keydown', eatSpace)
        return () => {
            document.removeEventListener('keyup', getSpace);
            document.removeEventListener('keydown', eatSpace);
        };
    }, [cases]);

    return (
        <div className="trainer">
            <LL
                case_={trainerCase.case}
                width={LLSize}
                height={LLSize}
            />
            <div>
                <button
                    onClick={loadTrainerCase}
                    className="newCaseBtn"
                >
                    change case
                </button>
                <div>
                    <span className="spacemsg grey">
                        use spacebar for a new case or{' '}
                    </span>
                    <Link to="/subsets">
                        choose a different subset
                    </Link>
                </div>
            </div>
            {!!solutionList.length && (
                <div>
                    <p className="setup">
                        <span className="grey">
                            setup - {getSetup(solutionList)}
                        </span>
                    </p>

                    <div className="solutions">
                        {solutionList.map((data, i) => (
                            <div key={i}>
                                <Moves data={data} />
                            </div>
                        ))}
                    </div>
                </div>
            )}
        </div>
    );
}
