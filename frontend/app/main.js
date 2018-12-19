import Worker from './worker';

const worker = new Worker();
worker.addEventListener('message', ({ data: { cube } }) => {

    // {"centers":["U","D","B","R","F","L"],
    //         "corners":[["U","L","B"],["U","B","R"],["U","R","F"],["U","F","L"],["D","B","L"],["D","R","B"],["D","F","R"],["D","L","F"]],
    //         "edges":[["U","B"],["U","R"],["U","F"],["U","L"],["B","L"],["B","R"],["F","R"],["F","L"],["D","B"],["D","R"],["D","F"],["D","L"]]}

let { centres, edges, corners } = cube;
corners = corners.map(d => d.join``)
edges = edges.map(d => d.join``)
console.log(cube);
    document.body.innerHTML = `
    <pre>
                ${edges[0][0]}
                ${centres[0]}
                ${edges[2][0]}
        </pre>
    `;
});

setTimeout(() => {
    worker.postMessage({ cube: 'anything here' });
}, 300);
