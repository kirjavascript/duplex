import React, { Fragment, useState, useCallback } from 'react';
import { useSolutions } from '#app/solver/store';
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
    const { solution } = data;
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

export default function Case({ case_, solutions, chosen }) {

    const { solving } = useSolutions();
    const [showModal, setShowModal] = useState(false);

    const closeModal = useCallback(() => {
        setShowModal(false);
    }, []);

    const rotate = chosen && chosen.solution[0] * 90;

    return (
        <div
            className="case visible"
        >
            <LL case_={case_} rotate={rotate} />
            <br />
            <pre>
                {chosen && <Moves data={chosen} trimAUF />}
            </pre>
            {solving ? (
                <pre>
                    ...
                </pre>
            ) : (
                <p>
                    {solutions.length} solutions
                    <br />
                    <span
                        onClick={() => setShowModal(true)}
                        className="modal-trigger"
                    >
                        {' '}(see all)
                    </span>
                </p>
            )}
            <Modal
                show={showModal}
                case_={case_}
                solutions={solutions}
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
                            {solutions.map((data, i) => (
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
