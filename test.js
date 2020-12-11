'use strict';
const fs = require('fs');
const WASI = require('wasi');
const wasi = new WASI({
  args: process.argv,
  env: process.env,
  preopens: {
    '/sandbox': __dirname
  }
});
const importObject = {
  wasi_snapshot_preview1: wasi.wasiImport,
  console,
};

(async () => {
  const wasm = await WebAssembly.compile(fs.readFileSync('./out.wasm'));
  const instance = await WebAssembly.instantiate(wasm, importObject);

  instance.exports.main();
})();
