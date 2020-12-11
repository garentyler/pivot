require("webassembly")
  .load("out.wasm")
  .then(module => {
    console.log("3 + 2 is " + module.exports.main());
  });
