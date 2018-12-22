/* eslint-disable */
const ENDPOINT = __DEV__
    ? 'http://lh:8000/duplex.wasm'
    : 'http://duplex.kirjava.xyz/duplex.wasm';

const stack = [];
fetch(ENDPOINT).then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, { env: {
        stack_push: (thing) => {
            stack.push(thing);
        },
        console_stack: (type) => {
            const method = ['log', 'warn', 'error'][type];
            console[method](
                '> ' + String.fromCharCode(...stack.splice(0, stack.length))
            );
            // const ref = {};
        }
    }})
).then(results => {
    // interop
    const { exports } = results.instance;
    exports.web_main();
    function getStringFromStack() {
        const [pointer, length] = [stack.pop(), stack.pop()];
        const buffer = new Uint8Array(
            exports.memory.buffer,
            pointer,
            length,
        );
        const string = String.fromCharCode(...buffer);
        exports.dealloc_rust_string(pointer);
        return string;
    }
    function createString(str) {
        const encoder = new TextEncoder();
        const encodedString = encoder.encode(str);
        const stringPtr = exports.alloc_js_string(encodedString.length);
        const mutStringPtr = exports.get_mut_js_string(stringPtr);
        const asBytes = new Uint8Array(
            exports.memory.buffer,
            mutStringPtr,
            encodedString.length,
        );
        asBytes.set(encodedString);
        return stringPtr;
    }
    const wasm = Object.keys(exports)
        .reduce((acc, cur) => {
            if (typeof exports[cur] === 'function') {
                acc[cur] = (...args) => {
                    const wrappedArgs = args
                        .map(arg => (
                            typeof arg === 'string' ? createString(arg) : arg
                        ));
                    exports[cur](...wrappedArgs);
                    if (stack.length) {
                        return getStringFromStack();
                    }
                };
            }
            return acc;
        }, {});

    // api


    // todo: console log stack to grid

    // console.log(getString('TEST_STRING'));

    // exports.receive_string(createString('this is a test'));

    self.onmessage = ({ data: { action, payload } }) => {
        if (action === 'LOAD_ALGS') {
            wasm.load_algs(JSON.stringify(payload));
            wasm.solve_alg('RUR\'URU2R\'');
        }
    };

    self.postMessage({ action: 'INIT' });
});
