/* eslint-disable */
const ENDPOINT = __DEV__
    ? 'http://lh:8000/duplex.wasm'
    : 'http://duplex.kirjava.xyz/duplex.wasm';

const stack = [];
const ref = { console_stack(method) { console.error('unreachable'); } };

fetch(ENDPOINT).then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, { env: {
        stack_push: (thing) => {
            stack.push(thing);
        },
        console_stack: (type) => {
            const method = ['log', 'warn', 'error'][type];
            ref.console_stack(method);
        }
    }})
).then(results => {
    // interop

    const { exports } = results.instance;
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

    // init console, call main

    ref.console_stack = (method) => {
        const message = getStringFromStack();
        console[method]('> ' + message);
    };
    exports.web_main();

    // wrap exports object

    const wasm = Object.keys(exports)
        .reduce((acc, cur) => {
            if (typeof exports[cur] === 'function') {
                acc[cur] = (...args) => {
                    const wrappedArgs = args
                        .map(arg => (
                            typeof arg === 'string' ? createString(arg) : arg
                        ));
                    const ret = exports[cur](...wrappedArgs);
                    if (stack.length > 2) {
                        throw new Error('export_string must only be called once per function');
                    } else if (stack.length) {
                        return getStringFromStack();
                    } else {
                        return ret;
                    }
                };
            }
            return acc;
        }, {});

    // userland

    self.onmessage = ({ data: { action, payload } }) => {
        if (action === 'LOAD_ALGS') {
            wasm.load_algs(JSON.stringify(payload));
        } else if (action === 'EXPLORE_SOLVE') {
            console.time('solve');
            wasm.explore_solve(JSON.stringify(payload));
            console.timeEnd('solve');
        }
    };

    self.postMessage({ action: 'INIT' });

    const cases = JSON.parse(wasm.enumerate_ll());

    self.postMessage({ action: 'CASES', payload: cases });

}).catch(console.error);
