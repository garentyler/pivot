/**
 * @module parser
 * @file Manages the parsing phase of Pivot.
 * @author Garen Tyler <garentyler@gmail.com>
 * @requires module:types
 */
const Token = require('./types.js').Token;
const Group = require('./types.js').Group;
const tokenizer = require('./tokenizer.js');

/**
 * @function parse
 * @desc Takes in an array of tokens, and outputs an AST.
 * @param {Token[]} tokens The input tokens.
 * @returns {Token[]} The tokens structured in an AST.
 * @public
 */
function parse(tokens) {
  // Create our output array.
  let ast = tokens;

  // Add indexes and levels.
  ast = addIndexes(ast);
  ast = addLevels(ast);

  // Get the deepest level.
  let deepestLevel = getDeepestLevel(ast);

  // Loop through for each level.
  for (let currentLevel = deepestLevel; currentLevel > 0; currentLevel--) {
    console.log('looping for level ' + currentLevel);
    let groupBuffer = [];
    for (let j = 0; j < ast.length; j++) {
      // Create previousLevel and nextLevel.
      // let previousTokenLevel = 0;
      // if (typeof ast[j-1] != 'undefined')
      //   previousTokenLevel = ast[j-1].level;
      let nextTokenLevel = 0;
      if (typeof ast[j+1] != 'undefined')
        nextTokenLevel = ast[j+1].level;

      if (ast[j].level == currentLevel) {
        groupBuffer.push(ast[j]); // Add the token to the groupBuffer.
        if (ast[j].level > nextTokenLevel) {
          let g = new Group(groupBuffer[0].value, groupBuffer);
          g.index = g.tokens[0].index;
          g.level = g.tokens[0].level - 1; // -1 because the group is on the level below.
          ast.splice(g.tokens[0].index, g.tokens.length + 1, g);
          j = g.tokens[0].index;
          groupBuffer = [];
        }
      }

      // // Take care of falling edges.
      // if (ast[j].level > nextTokenLevel && ast[j].level == currentLevel) {
      //   // The first item in the group is always a delimiter, steal info from that.
      //   console.log(groupBuffer[0]);
      // }
    }
  }

  return ast;
}

/**
 * @function addIndexes
 * @desc Adds basic indexes to the tokens.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The tokens with indexes.
 * @private
 */
function addIndexes(tokens) {
  return tokens.map((t, i) => {
    t.index = i;
    return t;
  });
}

/**
 * @function addLevels
 * @desc Adds basic levels to the tokens. The levels are dependent on the delimiters.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The tokens with levels.
 * @private
 */
function addLevels(tokens) {
  let level = 0;
  tokens = tokens.map((t, i) => {
    if (t.type == 'delimiter' && t.subtype == 'left')
      level++;
    t.level = level;
    if (t.type == 'delimiter' && t.subtype == 'right')
      level--;
    return t;
  });
  if (level > 0)
    throw new SyntaxError('Missing closing delimiter');
  else if (level < 0)
    throw new SyntaxError('Missing opening delimiter');
  else
    return tokens;
}

/**
 * @function getDeepestLevel
 * @desc Finds the deepest level.
 * @param {Token[]} tokens The tokens.
 * @returns {Number} The deepest level.
 * @private
 */
function getDeepestLevel(tokens) {
  return tokens.reduce((deepest, t) => {
    return t.level > deepest ? t.level : deepest;
  }, 0);
}

module.exports = {
  parse,
  util: {
    addIndexes,
    addLevels,
    getDeepestLevel
  }
};

require('fs').writeFileSync('ast.json', JSON.stringify(parse(tokenizer.tokenize('let x = (5 + (6 * 2));')), null, 2), () => {});
