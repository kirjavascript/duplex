const stack = [];
fetch('http://lh:8000/duplex.wasm').then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, { env: {
        stack_push: (thing) => {stack.push(thing);},
        console_stack: (type) => {
            const method = ['log', 'warn', 'error'][type];
            console[method](
                String.fromCharCode(...stack.splice(0, stack.length))
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

    const get_cube = getStringFunc('get_cube');
    const get_ll = getStringFunc('get_ll');

    // console.log(get_cube());
    console.log(get_ll());

    // todo: console log stack to grid

    // examples

    // console.log(getString('TEST_STRING'));

    // exports.receive_string(createString('this is a test'));

    // self.onmessage = ({ data: { cube } }) => {
    //     self.postMessage({ cube: getString('TEST_STRING') });
    // };
});
