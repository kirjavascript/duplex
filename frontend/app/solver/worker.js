/* eslint-disable */
const ENDPOINT = location.origin + '/duplex.wasm';

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
        exports.dealloc_str(pointer);
        return string;
    }
    function joinStringsFromStack() {
        const strings = [];
        while (stack.length) {
            strings.unshift(getStringFromStack());
        }
        return strings.join('');
    }
    function createString(str) {
        const encoder = new TextEncoder();
        const encodedString = encoder.encode(str);
        const stringPtr = exports.alloc_str(encodedString.length);
        const mutStringPtr = exports.get_mut_str(stringPtr);
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
        const message = joinStringsFromStack();
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
                    if (stack.length) {
                        return joinStringsFromStack();
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
            console.time('solution time');
            const error = wasm.load_algs(JSON.stringify(payload));
            if (error) {
                self.postMessage({ action: 'PARSE_ERROR', payload: error });
            } else {
                const solutions = JSON.parse(wasm.run_algs());
                self.postMessage({ action: 'SOLUTIONS', payload: solutions });
            }
            console.timeEnd('solution time');
        } else if (action === 'EXPLORE_SOLVE') {
            wasm.explore_solve(JSON.stringify(payload));
        } else if (action == 'LOAD_SUBSET') {
            wasm.load_subset(JSON.stringify(payload));
        }
    };

    self.postMessage({ action: 'INIT' });

    const cases = JSON.parse(wasm.enumerate_ll());

    self.postMessage({ action: 'CASES', payload: cases });

}).catch(console.error);
