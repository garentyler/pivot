function isLetter(char) {
  return /[A-Za-z]/.test(char);
}
function isOperator(char) {
  return /\+|\-|\*|\/|\=|\=\=|\>|\<|\>\=|\<\=|\=\>|\;/.test(char);
}
function isDigit(char) {
  return /\d/.test(char);
}
function isWhitespace(char) {
  return /\s/.test(char);
}
function Token(type, subtype, value) {
  this.type = type;
  this.subtype = subtype;
  this.value = value;
}
function opType(operator) {
  if (false)
    return 'left';
  else if (false)
    return 'right';
  else if (/\;/.test(operator))
    return 'none';
  else
    return 'dual';
}
function isKeyword(value) {
  return value == 'let';
}
function oldTokenize(rawCode) {
  let chars = rawCode.split('');
  // Create letter, operator, number, and string buffers.
  let lb = [];
  let ob = [];
  let nb = [];
  let sb = [];
  // Create token array.
  let tokens = [];
  let stringData = {
    inString: false,
    stringType: null
  };
  // Check for comments.
  for (let i = 0; i < chars.length - 1; i++) {
    let char = chars[i];
    if (char == '/' && chars[i + 1] == '/') {
      // Remove the characters from the comment.
      chars.splice(i, chars.indexOf('\n', i) - i);
      // Adjust the index accordingly.
      i -= chars.indexOf('\n', i) - i;
    }
    if (char == '/' && chars[i + 1] == '*') {
      // If multiline comment /* find the next */
      let endindex;
      for (let j = 0; j < chars.length - 1; j++) {
        if (chars[j] == '*' && chars[j + 1] == '/') {
          endindex = j + 2;
          console.log(chars.splice(i, (j + 2) - i));
        }
      }
      // Adjust the index accordingly.
      i -= endindex - i;
    }
  }
  // Check for characters to be escaped.
  for (let i = 0; i < chars.length; i++) {
    let char = chars[i];
    if ((char == '\\') && (i + 1 != chars.length)) {
      chars.splice(i, 2, `${char}${chars[i+1]}`);
      continue;
    }
  }
  // Loop through all the characters.
  for (let i = 0; i < chars.length; i++) {
    let char = chars[i];
    // Behave differently in/out of a string.
    if (stringData.inString) {
      if (char == `'`) {
        stringData.inString = false;
        tokens.push(new Token('string', 'n/a', sb.join('')));
        sb = [];
      } else {
        sb.push(char);
      }
    } else {
      if (char == `'`) {
        stringData.inString = true;
      } else {
        // Parsing chars, ignoring strings.
        if (isLetter(char)) {
          lb.push(char);
          if (ob.length > 0) {
            let op = ob.join('');
            tokens.push(new Token('operator', opType(op), op));
            ob = [];
          }
          if (nb.length > 0) {
            tokens.push(new Token('number', 'n/a', nb.join('')));
            nb = [];
          }
        } else if (isOperator(char)) {
          ob.push(char);
          if (lb.length > 0) {
            tokens.push(new Token('variable', 'n/a', lb.join('')));
            lb = [];
          }
          if (nb.length > 0) {
            tokens.push(new Token('number', 'n/a', nb.join('')));
            nb = [];
          }
        } else if (isDigit(char)) {
          nb.push(char);
          if (lb.length > 0) {
            tokens.push(new Token('variable', 'n/a', lb.join('')));
            lb = [];
          }
          if (ob.length > 0) {
            let op = ob.join('');
            tokens.push(new Token('operator', opType(op), op));
            ob = [];
          }
        } else if (isWhitespace(char)) {
          // Close all buffers.
          if (lb.length > 0) {
            tokens.push(new Token('variable', 'n/a', lb.join('')));
            lb = [];
          }
          if (ob.length > 0) {
            let op = ob.join('');
            tokens.push(new Token('operator', opType(op), op));
            ob = [];
          }
          if (nb.length > 0) {
            tokens.push(new Token('number', 'n/a', nb.join('')));
            nb = [];
          }
        } else if (isDelimiter(char)) {
          // Close all buffers.
          if (lb.length > 0) {
            tokens.push(new Token('variable', 'n/a', lb.join('')));
            lb = [];
          }
          if (ob.length > 0) {
            let op = ob.join('');
            tokens.push(new Token('operator', opType(op), op));
            ob = [];
          }
          if (nb.length > 0) {
            tokens.push(new Token('number', 'n/a', nb.join('')));
            nb = [];
          }
          // Categorize and push.
          if (char == '(' || char == ')') {
            tokens.push(new Token('delimiter', char == '(' ? 'left' : 'right', 'parenthesis'));
          } else if (char == '[' || char == ']') {
            tokens.push(new Token('delimiter', char == '[' ? 'left' : 'right', 'bracket'));
          } else if (char == '{' || char == '}') {
            tokens.push(new Token('delimiter', char == '{' ? 'left' : 'right', 'brace'));
          }
        }
      }
    }
  }
  // Check for keywords.
  for (let i = 0; i < tokens.length; i++) {
    if (tokens[i].type == 'variable' && isKeyword(tokens[i].value)) {
      tokens[i].type = 'keyword';
    }
  }
  // Add indexes.
  let layer = 0;
  for (let i = 0; i < tokens.length; i++) {
    tokens[i].index = i;
  }
  return tokens;
};
console.log(oldTokenize(`let x = 'it\\'s cool outside';`));
