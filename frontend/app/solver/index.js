import Worker from './worker';
let worker;

export function startWorker({ onload, cases }) {
    worker = new Worker();

    worker.addEventListener('message', ({ data: { action, payload } }) => {
        if (action === 'INIT') {
            onload();
        } else if (action === 'CASES') {
            cases(payload);
        }
    });
    return worker;
}


export function loadAlgs(algs) {
    worker.postMessage({
        action: 'LOAD_ALGS',
        payload: algs.map(({...args}, i) => ({index: i, ...args})),
    });
}

export function exploreSolve(transform) {
    worker.postMessage({
        action: 'EXPLORE_SOLVE',
        payload: transform,
    });
}
