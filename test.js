require("webassembly")
  .load("out.wasm")
  .then(module => {
    console.log("2 + 2 is " + module.exports.add(2, 2));
  });
