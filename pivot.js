// The Pivot code to run.
var code = `asdf = (a) {return(a++)}`;

// Import the tokenizer and parser.
const tokenize = require("./tokenizer.js");
const parse = require("./parser.js");

// Generate the AST.
var ast = parse(tokenize(code));

// Write the AST to ast.json.
var fs = require("fs");
fs.writeFileSync("ast.json", JSON.stringify(ast, null, 4));
