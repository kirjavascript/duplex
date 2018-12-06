// import Duplex from  './cube';

// Duplex((obj) => {
//     console.log(obj);
// });

import(/* webpackChunkName: "worker" */ './worker')
    .then(({ default: Worker }) => {
        const worker = new Worker();
        worker.addEventListener('message', ({ data: { cube } }) => {
            console.log(cube);
            // worker.terminate();
        });
        worker.postMessage({ cube: 'anything here' });
    })
    .catch(console.error);
