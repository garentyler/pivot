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

module.exports = {
  Token,
  Group
};
