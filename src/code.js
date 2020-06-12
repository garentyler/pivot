/**
 * @module code
 * @file Runs the code / transpiles the code to JavaScript
 * @author Garen Tyler <garentyler@gmail.com>
 */

 /**
  * @function translate
  * @desc Translates the code to JS, given an AST
  * @param {Token[]} ast The ast.
  * @returns {String} The JS code.
  * @public
  */
function translate(ast, data) {
  let out = '';
  for (let i = 0; i < ast.length; i++) {
    if (ast[i].type == 'operator' && ast[i].subtype == 'function call') {
      let temp = '';
      if (!(Object.keys(data).indexOf(ast[i].operands[0].value) > -1))
        throw new ReferenceError(`Undefined function ${ast[i].operands[0].value}`);
      else temp += data[ast[i].operands[0].value];
      temp += '(';
      for (let j = 0; j < ast[i].operands[1].tokens.length; j++) {
        if (j != 0)
          temp += ', ';
        if (ast[i].operands[1].tokens[j].type == 'string')
          temp += `"${ast[i].operands[1].tokens[j].value}"`;
      }
      temp += ');'
      out += temp;
    }
  }
  return {
    data,
    code: out
  };
}

module.exports = {
  translate
};
