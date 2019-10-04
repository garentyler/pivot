/**
 * @module parser
 * @file Manages the parser phase of Pivot.
 * @author Garen Tyler <garentyler@gmail.com>
 * @requires module:types
 */
const Token = require('./types.js').Token;
const Group = require('./types.js').Token;

module.exports = function(tokens) {
  let level = 0;
  // Add level markers.
  tokens.forEach((t, i) => {
    if (t.type == 'delimiter' && t.subtype == 'left') {
      tokens[i].level = level;
      level++;
    } else if (t.type == 'delimiter' && t.subtype == 'right') {
      level--;
      tokens[i].level = level;
    } else {
      tokens[i].level = level;
    }
  });
  // Group.
  let currentLevel = 0;
  let groupStack = [0];
  tokens.forEach((t, i) => {
    if (currentLevel < t.level) {
      tokens.splice(i, 0, new Group(tokens[i - 1].value, []));
      groupStack.push(i);
      currentLevel++;
      tokens[i].level = currentLevel;
    }
    if (t.level != 0) {
      tokens[groupStack.slice(-1)].tokens.push(t);
    }
    if (currentLevel > t.level) {
      groupStack.pop();
      currentLevel--;
    }
  });
  if (currentLevel != 0) {} // Error: missing delimiter.
  return tokens;
};
