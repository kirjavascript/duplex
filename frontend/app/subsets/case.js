import React, { Fragment, useState, useCallback } from 'react';
import Modal from './modal';
import LL from './ll';
import { useCases } from './store';

const auf = ['','U ','U2 ','U\' '];

function getName(solution) {
    return (
        <span className="blue">
            {solution.invert?'invert ':''}{solution.mirror?'mirror ':''}
            {solution.name}
        </span>
    );
}

export function Moves({ data, trimAUF }) {
    const { alg: solution } = data;
    const [showName, setName] = useState(true);
    return (
        <span
            onClick={() => { setName(!showName); }}
            className="moves"
        >
            {!trimAUF && auf[solution[0]]}
            {showName ? getName(solution[1]) : (
                <span className="blue">{solution[1].moves}</span>
            )}
            {solution.length === 4 && <Fragment>
                {'\n'}{auf[solution[2]]}
                {showName ? getName(solution[3]) :  (
                    <span className="blue">{solution[3].moves}</span>
                )}
            </Fragment>}
        </span>
    );
}

export default function Case({ case: case_, solutionIndices, trainer, index }) {

    const { solving, solutions, toggleTrainer } = useCases();
    const [showModal, setShowModal] = useState(false);

    const closeModal = useCallback(() => {
        setShowModal(false);
    }, []);

    const caseSolutions = solutionIndices
        ? solutionIndices.map(i => solutions[i])
        : [];

    const chosen = caseSolutions[0];

    const rotate = chosen ? (90 * chosen.alg[0]) : undefined;

    return (
        <div
            className="case visible"
            data-index={case_.index}
        >
            <LL case_={case_} rotate={rotate} />
            <br />
            {solving ? (
                <pre>
                    ...
                </pre>
            ) : (
                <Fragment>
                    <pre>
                        {chosen && <Moves data={chosen} trimAUF />}
                    </pre>
                    <p>
                        {caseSolutions.length} solutions
                        <span
                            onClick={() => setShowModal(true)}
                            className="modal-trigger"
                        >
                            {' '}(see all)
                        </span>
                        <br />
                        <span
                            className="checkbox trainer-checkbox"
                            onClick={() => {
                                toggleTrainer(index);
                            }}
                        >
                            trainer
                            <span className="link">
                                {trainer ? '✔' : '✗'}
                            </span>
                        </span>
                    </p>

                </Fragment>
            )}
            <Modal
                show={showModal}
                case_={case_}
            >
                {() => (
                    <Fragment>
                        <LL case_={case_} />
                        <button
                            type="button"
                            onClick={closeModal}
                            className="close"
                        >
                            close
                        </button>
                        <div className="solutions">
                            {caseSolutions.map((data, i) => (
                                <Fragment key={i}>
                                    <Moves data={data} />
                                </Fragment>
                            ))}
                        </div>
                    </Fragment>
                )}
            </Modal>
        </div>
    )
}
