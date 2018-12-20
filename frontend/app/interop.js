import Worker from './worker';

const worker = new Worker();
worker.addEventListener('message', ({ data: { cube } }) => {
    console.log(cube);
});

setTimeout(() => {
    worker.postMessage({ cube: 'anything here' });
}, 300);
