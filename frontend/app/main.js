// import Duplex from  './cube';

// Duplex((obj) => {
//     console.log(obj);
// });


import Worker from './worker';
const worker = new Worker();
worker.addEventListener('message', ({ data: { cube } }) => {
    console.log(cube);
    // worker.terminate();
});

worker.postMessage({ cube: 'anything here' });
