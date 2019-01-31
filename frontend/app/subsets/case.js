import React, { Fragment, useState, useCallback } from 'react';
// import normalize from 'cube-notation-normalizer';
import Modal from './modal';
import LL from './ll';
import { useCases } from './store';
import { solutionToString } from '../trainer';

const auf = ['','U ','U2 ','U\' '];

function getName(solution) {
    return (
        <span className="blue">
            {solution.invert?'invert ':''}{solution.mirror?'mirror ':''}
            {solution.name}
        </span>
    );
}

export function Moves({ data }) {
    const { solution } = data;
    const [showName, setName] = useState(true);
    return (
        <span
            onClick={() => { setName(!showName); }}
            className="moves"
        >
            {auf[solution[0]]}
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

    const star = solutions.find(({solution}) => {
        return solution.length === 2;
    });
    const ranked = solutions.map((data) => {
        const { solution: [auf0, solution0, auf1, solution1 = {}] } = data;

        const weight = select === 'transform'
            ? (!!solution0.mirror + !!solution0.invert
                + !!solution1.mirror + !!solution1.invert)
            : solutionToString(data).split(' ').length;

        return { weight, data };
    }).sort((a, b) => a.weight - b.weight);

    return {
        star,
        best: ranked.length ? ranked[0].data : undefined,
    };
}

export default function Case({ case_, solutions }) {

    const { best } = findSolution(solutions);
    const [showModal, setShowModal] = useState(false);

    const closeModal = useCallback(() => {
        setShowModal(false);
    }, []);

    return (
        <div
            className="case visible"
        >
            <LL case_={case_} />
            <br />
            <pre>
                {best && <Moves data={best} />}
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
