import React, { Fragment, useEffect } from 'react';
import { Link } from 'react-router-dom';
import normalize from 'cube-notation-normalizer';
import { useSolver } from '#app/solver';
import { useSolutions } from '#app/solver/store';
import LL from '#app/subsets/ll';
import { useWindowSize } from '#app/subsets/render-hooks';
import { Moves } from '#app/subsets/case';
import { useTrainer } from './store';

const auf = ['', 'U', 'U2', 'U\''];

function solutionToString(solution) {
    return solution.solution.map(part => {
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

    const { trainerCase } = useTrainer();
    const { loadTrainerCase, worker } = useSolver();
    const { solutions: solutionsList } = useSolutions();
    const { width } = useWindowSize();
    const solutions = solutionsList[trainerCase.index] || [];

    const LLSize = Math.min(400, width - 40);

    useEffect(() => {
        if (worker) {
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
        }
        return undefined;
    }, [worker]);

    return (
        <div className="trainer">
            <LL
                case_={trainerCase}
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
            {!!solutions.length && (
                <div>
                    <p className="setup">
                        <span className="grey">
                            setup - {getSetup(solutions)}
                        </span>
                    </p>

                    <div className="solutions">
                        {solutions.map((data, i) => (
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
