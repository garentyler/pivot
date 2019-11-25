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
  ast = postfixOperators(ast);
  // Precedence 12.
  ast = prefixOperators(ast);
  // Precedence 11.
  ast = mathOperators(ast, 0); // Level 0 math operators: **.
  // Precedence 10.
  ast = mathOperators(ast, 1); // Level 1 math operators: *, /, %.
  // Precedence 9.
  ast = mathOperators(ast, 2); // Level 2 math operators: +, -.
  // Precedence 7.
  // ast = comparisonOperators(ast);
  // Precedence 6.
  // ast = assign(ast);
  // Precedence 4.
  // ast = logicOperators(ast);
  // Precedence 3.
  // ast = opAssign(ast);
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
  let groupBuffer;
  let opening;
  // Group the deepest levels first.
  for (let currentLevel = deepestLevel; currentLevel > 0; currentLevel--) {
    groupBuffer = []; // Overwrite groupBuffer.
    opening = null; // Overwrite opening.
    for (let i = 0; i < tokens.length; i++) {
      if (tokens[i].level == currentLevel) {
        if (groupBuffer.length == 0)
          opening = tokens[i];
        groupBuffer.push(tokens[i]);
        if (typeof tokens[i + 1] == 'undefined' || (tokens[i].type == 'delimiter' && tokens[i].subtype == 'right' && tokens[i].value == opening.value)) { // The end of the tokens.
          let g = new Group(groupBuffer[0].value, groupBuffer);
          g.index = g.tokens[0].index;
          g.level = g.tokens[0].level - 1; // -1 because the group is on the level below.
          let length = g.tokens.length; // Keep track of how many tokens there are before removing the delimiters.
          g.tokens = g.tokens.splice(1, g.tokens.length - 2); // Remove the delimiters in g.tokens.
          i -= length - 1; // Reset the counter.
          tokens.splice(i, length, g); // Replace the tokens with the new group.
          // Reset groupBuffer and opening.
          groupBuffer = [];
          opening = null;
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
function memberAccess(ast) {
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
  for (let i = 0; i < ast.length; i++) {
    if (ast[i].type == 'group')
      ast[i].tokens = functionCreation(ast[i].tokens); // Recursively order the groups.
    if (typeof ast[i + 1] == 'undefined')
      continue; // Skip this loop.
    if ((ast[i].type == 'group' && ast[i].subtype == 'parenthesis') && (ast[i + 1].type == 'group' && ast[i + 1].subtype == 'brace')) {
      // Parenthesis group followed by brace group.
      ast[i + 1].tokens = functionCreation(ast[i + 1].tokens); // Order the other group before we do anything.
      let op = new Operator('function creation', 'n/a', [ast[i], ast[i + 1]]);
      op.index = ast[i].index;
      op.level = ast[i].level;
      ast.splice(i, 2, op);
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
    if (ast[i].type == 'group') {
      if (ast[i].tokens.length > 0) {
        ast[i].tokens = postfixOperators(ast[i].tokens);
      }
    } else if (ast[i].type == 'operator') {
      if (typeof ast[i].operands != 'undefined') {
        ast[i].operands = postfixOperators(ast[i].operands);
      }
    }
    if (ast[i].type == 'operator' && ast[i].subtype == 'postfix') { // The operand is on the left.
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
  for (let i = ast.length - 1; i >= 0; i--) { // Prefix operators are rtl associative, so loop backwards.
    // Take care of the tokens in the groups.
    if (ast[i].type == 'group') {
      if (ast[i].tokens.length > 0) {
        ast[i].tokens = prefixOperators(ast[i].tokens);
      }
    } else if (ast[i].type == 'operator') {
      if (typeof ast[i].operands != 'undefined') {
        ast[i].operands = prefixOperators(ast[i].operands);
      }
    }
    if (ast[i].type == 'operator' && ast[i].subtype == 'prefix') { // The operand is on the right.
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

/**
 * @function mathOperators
 * @desc Recursively structures the math operators.
 * @param {Token[]} ast The ast.
 * @param {Token[]} level The level of math to do. (Order of operations)
 * @returns {Token[]} The ast with structured math operators.
 * @private
 */
function mathOperators(ast, level) {
  if (level == 0) { // Level 0 operators: **
    for (let i = ast.length - 1; i >= 0; i--) { // Exponentiation is rtl associative, so loop backwards.
      // Take care of the tokens in the groups.
      if (ast[i].type == 'group') {
        if (ast[i].tokens.length > 0) {
          ast[i].tokens = mathOperators(ast[i].tokens, level);
        }
      } else if (ast[i].type == 'operator') {
        if (typeof ast[i].operands != 'undefined') {
          ast[i].operands = mathOperators(ast[i].operands, level);
        }
      }
      if (ast[i].type == 'operator' && ast[i].value == '**') {
        if (typeof ast[i - 1] == 'undefined' || typeof ast[i + 1] == 'undefined')
          throw new SyntaxError('Dual operator requires two operands.');
        let op = new Operator('dual', ast[i].value, [ast[i - 1], ast[i + 1]]);
        op.index = ast[i].index;
        op.level = ast[i].level;
        ast.splice(i - 1, 3, op);
        i--;
      }
    }
  } else {
    for (let i = 0; i < ast.length; i++) { // All other math operators are ltr associative.
      // Take care of the tokens in the groups.
      if (ast[i].type == 'group') {
        if (ast[i].tokens.length > 0) {
          ast[i].tokens = mathOperators(ast[i].tokens, level);
        }
      } else if (ast[i].type == 'operator') {
        if (typeof ast[i].operands != 'undefined') {
          ast[i].operands = mathOperators(ast[i].operands, level);
        }
      }
      if (level == 1) {
        if (ast[i].type == 'operator' && (ast[i].value == '*' || ast[i].value == '/' || ast[i].value == '%')) {
          if (typeof ast[i - 1] == 'undefined' || typeof ast[i + 1] == 'undefined')
            throw new SyntaxError('Dual operator requires two operands.');
          let op = new Operator('dual', ast[i].value, [ast[i - 1], ast[i + 1]]);
          op.index = ast[i].index;
          op.level = ast[i].level;
          ast.splice(i - 1, 3, op);
          i--;
        }
      } else if (level == 2) {
        if (ast[i].type == 'operator' && (ast[i].value == '+' || ast[i].value == '-')) {
          if (typeof ast[i - 1] == 'undefined' || typeof ast[i + 1] == 'undefined')
            throw new SyntaxError('Dual operator requires two operands.');
          let op = new Operator('dual', ast[i].value, [ast[i - 1], ast[i + 1]]);
          op.index = ast[i].index;
          op.level = ast[i].level;
          ast.splice(i - 1, 3, op);
          i--;
        }
      }
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
