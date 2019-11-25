/**
 * @module parser
 * @file Manages the parsing phase of Pivot.
 * @author Garen Tyler <garentyler@gmail.com>
 * @requires module:types
 */
const Group = require('./types.js').Group;
const Operator = require('./types.js').Operator;

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

  // Start grouping by precedence
  ast = grouping(ast);
  ast = memberAccess(ast);
  ast = postfixOperators(ast);
  ast = prefixOperators(ast);
  console.log(ast);

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
 * @desc Finds the deepest level of the ast.
 * @param {Token[]} tokens The tokens.
 * @returns {Number} The deepest level.
 * @private
 */
function getDeepestLevel(tokens) {
  return tokens.reduce((deepest, t) => {
    return t.level > deepest ? t.level : deepest;
  }, 0);
}

/**
 * @function grouping
 * @desc Combine groups of tokens by delimiter.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The grouped tokens, or the basic ast.
 * @private
 */
function grouping(tokens) {
  // Get the deepest level.
  let deepestLevel = getDeepestLevel(tokens);
  // Loop through for each level.
  for (let currentLevel = deepestLevel; currentLevel > 0; currentLevel--) {
    let groupBuffer = [];
    for (let j = 0; j < tokens.length; j++) {
      let nextTokenLevel = 0;
      if (typeof tokens[j + 1] != 'undefined')
        nextTokenLevel = tokens[j + 1].level;
      if (tokens[j].level == currentLevel) {
        groupBuffer.push(tokens[j]); // Add the token to the groupBuffer.
        if (tokens[j].level > nextTokenLevel) {
          let g = new Group(groupBuffer[0].value, groupBuffer);
          g.index = g.tokens[0].index;
          g.level = g.tokens[0].level - 1; // -1 because the group is on the level below.
          tokens.splice(g.tokens[0].index, g.tokens.length, g);
          j = g.tokens[0].index;
          // Remove the delimiters in g.tokens.
          g.tokens = g.tokens.splice(1, groupBuffer.length - 2);
          groupBuffer = [];
        }
      }
    }
  }
  return tokens;
}

/**
 * @function memberAccess
 * @desc Combine groups of tokens by member access.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The grouped tokens, or the basic ast.
 * @private
 */
function memberAccess(ast) {
  for (let i = 0; i < ast.length; i++) {
    if (ast[i].type == 'group')
      memberAccess(ast[i].tokens); // Recursively order the groups.
    else if (ast[i].type == 'operator' && ast[i].value == '.') { // Member access operator.
      if (typeof ast[i - 1] == 'undefined' || typeof ast[i + 1] == 'undefined')
        throw new SyntaxError('Operator requires two operands.');
      let op = new Operator(ast[i].subtype, ast[i].value, [ast[i - 1], ast[i + 1]]);
      op.index = ast[i - 1].index;
      op.level = ast[i].level;
      ast.splice(i - 1, 3, op);
      i--; // Removed 3 tokens, put in 1, skip 1 token. Reduce the counter by 1.
    }
  }
  return ast;
}

/**
 * @function postfixOperators
 * @desc Recursively structures the postfix operators.
 * @param {Token[]} ast The ast.
 * @returns {Token[]} The ast with structured postfix operators.
 * @private
 */
function postfixOperators(ast) {
  for (let i = 0; i < ast.length; i++) {
    // Take care of the tokens in the groups.
    if (ast[i].type == 'group')
      ast[i].tokens = postfixOperators(ast[i].tokens);
    else if (ast[i].type == 'operator' && ast[i].subtype == 'postfix') { // The operand is on the left.
      if (typeof ast[i - 1] == 'undefined')
        throw new SyntaxError('Postfix operator requires one operand before it.');
      let op = new Operator(ast[i].subtype, ast[i].value, [ast[i - 1]]);
      op.index = ast[i].index;
      op.level = ast[i].level;
      ast.splice(i - 1, 2, op);
      // Removing 2 tokens, adding 1, skip 1 token. Don't reduce the counter.
    }
  }
  return ast;
}

/**
 * @function prefixOperators
 * @desc Recursively structures the prefix operators.
 * @param {Token[]} ast The ast.
 * @returns {Token[]} The ast with structured prefix operators.
 * @private
 */
function prefixOperators(ast) {
  for (let i = 0; i < ast.length; i++) {
    // Take care of the tokens in the groups.
    if (ast[i].type == 'group')
      ast[i].tokens = postfixOperators(ast[i].tokens);
    else if (ast[i].type == 'operator' && ast[i].subtype == 'prefix') { // The operand is on the right.
      if (typeof ast[i + 1] == 'undefined')
        throw new SyntaxError('Prefix operator requires one operand after it.');
      let op = new Operator(ast[i].subtype, ast[i].value, [ast[i + 1]]);
      op.index = ast[i].index;
      op.level = ast[i].level;
      ast.splice(i, 2, op);
      // Removing 2 tokens, adding 1, skip 1 token. Don't reduce the counter.
    }
  }
  return ast;
}

module.exports = {
  parse,
  util: {
    addIndexes,
    addLevels,
    getDeepestLevel,
    grouping,
  }
};

// require('fs').writeFileSync('ast.json', JSON.stringify(parse(tokenizer.tokenize('let x = (5 + (6 * 2)) - 7;')), null, 2), () => {});
