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

  // Start grouping by precedence.

  // Precedence 16.
  ast = grouping(ast);
  // Precedence 15.
  ast = memberAccess(ast);
  ast = computedMemberAccess(ast);
  ast = functionCall(ast);
  // Precedence 14.
  ast = keywords(ast);
  ast = functionCreation(ast);
  // Precedence 13.
  // Precedence 12.
  ast = postfixOperators(ast);
  ast = prefixOperators(ast);

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
 * @returns {Token[]} The ast with grouped member access.
 * @private
 */
// TODO: Member access
function memberAccess(ast) {
  console.log(ast);
  for (let i = 0; i < ast.length; i++) {
    if (ast[i].type == 'group')
      ast[i].tokens = memberAccess(ast[i].tokens); // Recursively order the groups.
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
 * @function computedMemberAccess
 * @desc Combine groups of tokens by computed member access.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The ast with grouped computed member access.
 * @private
 */
function computedMemberAccess(ast) {
  // Computed member access is Variable, Bracket Group.
  for (let i = 0; i < ast.length; i++) {
    if (ast[i].type == 'group')
      ast[i].tokens = computedMemberAccess(ast[i].tokens); // Recursively order the groups.
    else if (ast[i].type == 'name' && ast[i].subtype == 'variable') { // Member access operator.
      if (typeof ast[i + 1] == 'undefined')
        continue; // Nothing after the variable; skip this loop.
      if (ast[i + 1].type == 'group' && ast[i + 1].subtype == 'bracket') {
        ast[i + 1].tokens = computedMemberAccess(ast[i + 1].tokens); // Order the group that we care about before we mess with it.
        let op = new Operator('n/a', 'member access', [ast[i], ast[i + 1]]);
        op.index = ast[i].index;
        op.level = ast[i].level;
        ast.splice(i, 2, op);
        // Removed 2 tokens, put in 1, skip 1 token. Don't reduce the counter.
      } else continue; // Not what we need.
    }
  }
  return ast;
}

/**
 * @function functionCall
 * @desc Combine groups of tokens by function calls.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The ast with grouped function calls.
 * @private
 */
function functionCall(ast) {
  // Function call is Variable, Parenthesis Group.
  for (let i = 0; i < ast.length; i++) {
    if (ast[i].type == 'group')
      ast[i].tokens = functionCall(ast[i].tokens); // Recursively order the groups.
    else if (ast[i].type == 'name' && ast[i].subtype == 'variable') { // Member access operator.
      if (typeof ast[i + 1] == 'undefined')
        continue; // Nothing after the variable; skip this loop.
      if (ast[i + 1].type == 'group' && ast[i + 1].subtype == 'parenthesis') {
        ast[i + 1].tokens = functionCall(ast[i + 1].tokens); // Order the group that we care about before we mess with it.
        let op = new Operator('function call', ast[i].value, [ast[i], ast[i + 1]]);
        op.index = ast[i].index;
        op.level = ast[i].level;
        ast.splice(i, 2, op);
        // Removed 2 tokens, put in 1, skip 1 token. Don't reduce the counter.
      } else continue; // Not what we need.
    }
  }
  return ast;
}

/**
 * @function keywords
 * @desc Combine groups of tokens by keywords.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The ast with grouped keywords.
 * @private
 */
function keywords(ast) {
  for (let i = ast.length - 1; i >= 0; i--) { // Keywords are rtl associative, so loop backwards.
    if (ast[i].type == 'group')
      ast[i].tokens = keywords(ast[i].tokens); // Recursively order the groups.
    else if (ast[i].type == 'name' && ast[i].subtype == 'keyword') {
      if (typeof ast[i + 1] == 'undefined')
        throw new SyntaxError('Keyword requires one operand after it.');
      let key = new Operator('keyword', ast[i].value, [ast[i + 1]]);
      key.level = ast[i].level;
      key.index = ast[i].index;
      ast.splice(i, 2, key);
      // Looping backwards and didn't remove any items before the current one. Don't reduce the counter.
    }
  }
  return ast;
}

/**
 * @function functionCreation
 * @desc Combine groups of tokens by function creation.
 * @param {Token[]} tokens The tokens.
 * @returns {Token[]} The ast with grouped function creation.
 * @private
 */
function functionCreation(ast) {
  // Function call is Parenthesis Group, Brace Group.
  console.log(ast);
  for (let i = 0; i < ast.length; i++) {
    if (ast[i].type == 'group')
      ast[i].tokens = functionCreation(ast[i].tokens); // Recursively order the groups.
    else if (ast[i].type == 'group' && ast[i].subtype == 'parenthesis') {
      if (typeof ast[i + 1] == 'undefined')
        continue; // Nothing after this group, ignore it.
      else if (ast[i + 1] == 'group' && ast[i].subtype == 'brace') {
        ast[i + 1].tokens = functionCreation(ast[i + 1].tokens); // Order the group that we care about before we mess with it.
        let op = new Operator('n/a', 'function creation', [ast[i], ast[i + 1]]);
        op.index = ast[i].index;
        op.level = ast[i].level;
        ast.splice(i, 2, op);
        // Removed 2 tokens, put in 1, skip 1 token. Don't reduce the counter.
      } else continue;
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
      if (typeof ast[i - 1] == 'undefined')
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
