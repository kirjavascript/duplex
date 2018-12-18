const stack = [];
fetch('http://lh:8000/duplex.wasm').then(response =>
    response.arrayBuffer()
).then(bytes =>
    WebAssembly.instantiate(bytes, { env: {
        stack_push: (thing) => {stack.push(thing);},
        console_log_stack: () => {
            console.log(
                String.fromCharCode(...stack.splice(0, stack.length))
            );
        }
    }})
).then(results => {
    console.log(results);
    const { exports } = results.instance;
    function getString(str) {
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

    // examples

    console.log(getString('TEST_STRING'));

    exports.receive_string(createString('this is a test'));

    self.onmessage = ({ data: { cube } }) => {
        self.postMessage({ cube: getString('TEST_STRING') });
    };
});
