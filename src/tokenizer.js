/**
 * @module tokenizer
 * @file Manages the tokenization phase of Pivot.
 * @author Garen Tyler <garentyler@gmail.com>
 * @requires module:types
 */
const Token = require('./types.js').Token;
const Group = require('./types.js').Token;

/**
 * @function tokenize
 * @desc Takes in raw code, and outputs an array of Tokens.
 * @param {string} code The raw input code.
 * @returns {Token[]} The code, split into tokens.
 * @public
 */
function tokenize(code) {
  // Split the string into an array of chars.
  let chars = code.split('');

  // Create buffers.
  let letterBuffer = [];
  let operatorBuffer = [];
  let numberBuffer = [];
  let stringBuffer = [];

  // Create the output Token[].
  let tokens = [];

  // Create an object to keep track of string data.
  let stringData = {
    inString: false,
    stringType: null
  };

  // Escape chars and remove comments.
  chars = combineEscapedChars(chars);
  chars = removeComments(chars);


  // Actually tokenize the chars.
  for (let i = 0; i < chars.length; i++) {
    let char = chars[i];
    if (stringData.inString) { // Tokenize differently in a string.
      // If a string delimiter and the same as the inital delimiter.
      if (determineCharType(char) == 'string delimiter' && char == stringData.stringType) {
        stringData.inString = false; // Not in a string any more.
        tokens.push(new Token('string', 'n/a', stringBuffer.join(''))); // Push the string.
        stringBuffer = []; // Clear the string buffer.
      } else stringBuffer.push(char); // Add to the string buffer.
    } else { // Tokenize normally.
      if (determineCharType(char) == 'string delimiter') {
        stringData.inString = true; // In a string now.
        stringData.stringType = char;
      } else if (determineCharType(char) == 'letter') {
        letterBuffer.push(char); // Add to the letter buffer.
        // End the other buffers.
        if (operatorBuffer.length > 0) {
          let operator = operatorBuffer.join('');
          tokens.push(new Token('operator', operatorType(operator), operator));
          operatorBuffer = [];
        }
        if (numberBuffer.length > 0) {
          let number = numberBuffer.join('');
          tokens.push(new Token('number', 'n/a', number));
          numberBuffer = [];
        }
      } else if (determineCharType(char) == 'operator') {
        operatorBuffer.push(char); // Add to the operator buffer.
        // End the other buffers.
        if (letterBuffer.length > 0) {
          let variable = letterBuffer.join('');
          tokens.push(new Token('name', 'variable', variable));
          letterBuffer = [];
        }
        if (numberBuffer.length > 0) {
          let number = numberBuffer.join('');
          tokens.push(new Token('number', 'n/a', number));
          numberBuffer = [];
        }
      } else if (determineCharType(char) == 'digit') {
        numberBuffer.push(char); // Add to the number buffer.
        // End the other buffers.
        if (letterBuffer.length > 0) {
          let variable = letterBuffer.join('');
          tokens.push(new Token('name', 'variable', variable));
          letterBuffer = [];
        }
        if (operatorBuffer.length > 0) {
          let operator = operatorBuffer.join('');
          tokens.push(new Token('operator', operatorType(operator), operator));
          operatorBuffer = [];
        }
      } else if (determineCharType(char) == 'whitespace') {
        // End all buffers.
        if (letterBuffer.length > 0) {
          let variable = letterBuffer.join('');
          tokens.push(new Token('name', 'variable', variable));
          letterBuffer = [];
        }
        if (numberBuffer.length > 0) {
          let number = numberBuffer.join('');
          tokens.push(new Token('number', 'n/a', number));
          numberBuffer = [];
        }
        if (operatorBuffer.length > 0) {
          let operator = operatorBuffer.join('');
          tokens.push(new Token('operator', operatorType(operator), operator));
          operatorBuffer = [];
        }
      } else if (determineCharType(char) == 'delimiter') {
        // End all buffers.
        if (letterBuffer.length > 0) {
          let variable = letterBuffer.join('');
          tokens.push(new Token('name', 'variable', variable));
          letterBuffer = [];
        }
        if (numberBuffer.length > 0) {
          let number = numberBuffer.join('');
          tokens.push(new Token('number', 'n/a', number));
          numberBuffer = [];
        }
        if (operatorBuffer.length > 0) {
          let operator = operatorBuffer.join('');
          tokens.push(new Token('operator', operatorType(operator), operator));
          operatorBuffer = [];
        }
        // Push the delimiter.
        tokens.push(getDelimiterToken(char));
      }
    }
  }

  // Empty all the buffers.
  if (letterBuffer.length > 0) {
    let variable = letterBuffer.join('');
    tokens.push(new Token('name', 'variable', variable));
    letterBuffer = [];
  }
  if (numberBuffer.length > 0) {
    let number = numberBuffer.join('');
    tokens.push(new Token('number', 'n/a', number));
    numberBuffer = [];
  }
  if (operatorBuffer.length > 0) {
    let operator = operatorBuffer.join('');
    tokens.push(new Token('operator', operatorType(operator), operator));
    operatorBuffer = [];
  }

  tokens = changeKeywords(tokens);

  return tokens;
}

/**
 * @function combineEscapedChars
 * @desc Combines escaped chars into one char.
 * @param {string[]} chars The chars.
 * @returns {string[]} The chars with combined escaped chars.
 * @private
 */
function combineEscapedChars(chars) {
  // Check for characters to be escaped.
  for (let i = 0; i < chars.length; i++) {
    if (chars[i] == '\\') {
      chars.splice(i, 2, chars[i] + chars[i + 1]);
      i -= 2;
    }
  }
  return chars;
}

/**
 * @function removeComments
 * @desc Removes comments.
 * @param {string[]} chars The chars.
 * @returns {string[]} The chars without comments.
 * @private
 */
function removeComments(chars) {
  let inComment = false; // Keep track if in a comment.
  for (let i = 0; i < chars.length; i++) {
    if (chars[i] == '/') {
      if (chars[i + 1] == '/') {
        inComment = true;
      }
    }
    if (chars[i] == '\n') {
      inComment = false;
      chars.splice(i, 1); // Remove the newline at the end of the comment.
      i--;
    }
    if (inComment) {
      chars.splice(i, 1); // Remove the char in the comment.
      i--;
    }
  }
  return chars;
}

/**
 * @function changeKeywords
 * @desc Changes tokens with subtype variable to subtype keyword
 * @param {Token[]} tokens The tokens
 * @returns {Token[]} The tokens with keywords.
 * @private
 */
function changeKeywords(tokens) {
  return tokens.map(t => {
    if (t.subtype == 'variable' && determineType(t.value) == 'keyword') {
      t.subtype = 'keyword';
    }
    return t;
  });
}

/**
 * @function getDelimiterToken
 * @desc Turns a delimiter char into a token.
 * @param {string} delimiter The delimiter char.
 * @returns {Token} The delimiter token.
 * @private
 */
function getDelimiterToken(delimiter) {
  if (/\(|\)/.test(delimiter))
    return new Token('delimiter', delimiter == '(' ? 'left' : 'right', 'parenthesis');
  else if (/\[|\]/.test(delimiter))
    return new Token('delimiter', delimiter == '[' ? 'left' : 'right', 'bracket');
  else if (/\{|\}/.test(delimiter))
    return new Token('delimiter', delimiter == '{' ? 'left' : 'right', 'brace');
  else throw new Error('Expected delimiter but got ' + delimiter);
}

/**
 * @function operatorType
 * @desc Determines the type of operator.
 * @param {string} operator The operator char.
 * @returns {string} The type of operator.
 * @private
 */
function operatorType(operator) {
  // Left operators have parameters on the left.
  if (/\+\+|--/.test(operator))
    return 'left';
  else if (false)
    return 'right';
  else if (/\;/.test(operator))
    return 'none';
  else
    return 'dual';
}

/**
 * @function determineCharType
 * @desc Detects the type of characters.
 * @param {string} char The input character(s).
 * @returns {string} The type of char.
 * @private
 */
function determineCharType(char) {
  if (/[A-Za-z]/.test(char))
    return 'letter';
  else if (/\+|\-|\*|\/|\=|\=\=|\>|\<|\>\=|\<\=|\=\>|;/.test(char))
    return 'operator';
  else if (/\(|\)|\[|\]|\{|\}/.test(char))
    return 'delimiter';
  else if (/'|"|`/.test(char))
    return 'string delimiter';
  else if (/\d/.test(char))
    return 'digit';
  else if (/\\./.test(char))
    return 'escaped char';
  else if (/\s/.test(char))
    return 'whitespace';
  else throw new SyntaxError('Unexpected char ' + char);
};

/**
 * @function determineType
 * @desc Detects the type of a string.
 * @param {string} str The input string.
 * @returns {string} The type of string.
 * @private
 */
function determineType(str) {
  if (/let|return/.test(str))
    return 'keyword';
  else return 'unknown';
};

module.exports = {
  tokenize,
  util: {
    combineEscapedChars,
    removeComments,
    changeKeywords,
    getDelimiterToken,
    operatorType,
    determineCharType,
    determineType
  }
};
