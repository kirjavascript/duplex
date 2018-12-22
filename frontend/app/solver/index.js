import Worker from './worker';
let worker;
const callbacks = {};

export function startWorker(onload) {
    worker = new Worker();

    worker.addEventListener('message', ({ data: { action } }) => {
        if (action === 'INIT') {
            onload();
        }
    });
}


export function loadAlgs(algs) {
    worker.postMessage({
        action: 'LOAD_ALGS',
        payload: algs,
    });
}

export function exploreSolve(transform) {
    worker.postMessage({
        action: 'EXPLORE_SOLVE',
        payload: transform,
    });
}
