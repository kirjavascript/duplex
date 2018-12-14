const { getOptions } = require('loader-utils');
module.exports = function (source) {
    const { name, path } = getOptions(this);
    return `
        ${source.replace(
            new RegExp(`"${name}.wasm"`, 'g'),
            `"${path}${name}.wasm"`
        )};
        self.Rust = Rust;
    `;
};
