/**
 * @module types
 * @file Provides a consistent source of types for the compiler.
 * @author Garen Tyler <garentyler@gmail.com>
 */

/**
 * @class Token
 * @classdesc Stores the type of token, subtype, and literal char value.
 */
function Token(type, subtype, value) {
  this.type = type;
  this.subtype = subtype;
  this.value = value;
}

/**
 * @class Group
 * @classdesc Stores the type of group, and the tokens in the group.
 */
function Group(type, tokens) {
  this.type = 'group'
  this.subtype = type;
  this.tokens = tokens;
}

/**
 * @class Operator
 * @classdesc Stores the type of operator, and tokens for an operator.
 */
function Operator(subtype, value, operands) {
  this.type = 'operator';
  this.subtype = subtype;
  this.value = value;
  this.operands = operands;
}

module.exports = {
  Token,
  Group,
  Operator
};
