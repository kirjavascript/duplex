/* eslint-disable */
import interop from 'raw-loader!!../../target/wasm32-unknown-unknown/release/duplex';
// webworkers need to use absolute urls
eval(interop.replace(/"duplex.wasm"/g, '"http://[::1]:8000/duplex.wasm"'));
/* eslint-enable */

self.onmessage = ({ data: { cube } }) => {
    Rust.duplex // eslint-disable-line
        .then((obj) => {
            self.postMessage({ cube: obj.display() });
        })
        .catch(console.error);
};
