export default function(cb) {
    Rust.duplex // eslint-disable-line
        .then(cb)
        .catch(console.error);
}
