import '../../target/wasm32-unknown-unknown/release/duplex';

self.onmessage = ({ data: { cube } }) => {
    Rust.duplex // eslint-disable-line
        .then((obj) => {
            self.postMessage({ cube: obj.display() });
        })
        .catch(console.error);
};
