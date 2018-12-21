import Worker from './worker';
let worker;
const callbacks = {};

export function startWorker(onload) {
    worker = new Worker();

    worker.addEventListener('message', ({ data: { action } }) => {
        if (action == 'INIT') {
            onload();
        }
    });
}


export function updateAlgs(algs) {
    worker.postMessage({
        action: 'UPDATE_ALGS',
        payload: algs,
    });
}
