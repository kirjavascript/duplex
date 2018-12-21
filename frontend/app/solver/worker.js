const ENDPOINT = __DEV__ // eslint-disable-line
    ? 'http://lh:8000/duplex.wasm'
    : 'http://duplex.kirjava.xyz/duplex.wasm';

const stack = [];
fetch(ENDPOINT).then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, { env: {
        stack_push: (thing) => {stack.push(thing);},
        console_stack: (type) => {
            const method = ['log', 'warn', 'error'][type];
            console[method](
                '> ' + String.fromCharCode(...stack.splice(0, stack.length))
            );
        }
    }})
).then(results => {
    // interop

    const { exports } = results.instance;
    exports.web_main();
    function getStringFunc(str) {
        return () => {
            exports[str]();
            const [pointer, length] = [stack.pop(), stack.pop()];
            const buffer = new Uint8Array(
                exports.memory.buffer,
                pointer,
                length,
            );
            const string = String.fromCharCode(...buffer);
            exports.dealloc_rust_string(pointer);
            return string;
        };
    }
    function getJSONFunc(str) {
        const func = getStringFunc(str);
        return () => JSON.parse(func());
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
    function createJSON(str) {
        return createString(JSON.stringify(str));
    }

    // api

    const get_cube = getJSONFunc('get_cube');
    const get_ll = getStringFunc('get_ll');

    // todo: console log stack to grid

    // console.log(getString('TEST_STRING'));

    // exports.receive_string(createString('this is a test'));

    self.onmessage = ({ data: { action, payload } }) => {
        if (action === 'LOAD_ALGS') {
            exports.load_algs(createJSON(payload));
            // exports.solve_alg(createString('RUR\'URU2R\''));
        }
    };

    self.postMessage({ action: 'INIT' });
});
