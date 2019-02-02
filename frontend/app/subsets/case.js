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

function findSolution(solutions) {
    const { select } = useCases();

    const stars = solutions.filter(({solution}) => {
        return solution.length === 2;
    });
    const ranked = (stars.length ? stars : solutions).map((data) => {
        const {
            solution: [auf0, solution0, auf1, solution1 = { length: 0 }],
        } = data;

        const weight = select === 'transform'
            ? (!!solution0.mirror + !!solution0.invert
                + !!solution1.mirror + !!solution1.invert)
            : solution0.length + +!!auf0 + +!!auf1 + +solution1.length;

        return { weight, data };
    }).sort((a, b) => a.weight - b.weight);

    const best = ranked.length ? ranked[0].data : undefined;

    return best;
}

export default function Case({ case_, solutions }) {

    const chosen = findSolution(solutions);
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
