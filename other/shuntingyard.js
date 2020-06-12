const repl = require('./repl.js');
// My implementation of the algorithm from https://en.wikipedia.org/wiki/Shunting-yard_algorithm.
function shuntingYardSolve(exp) {
  exp = exp.split('');
  // Remove whitespace.
  exp = exp.filter(e => /\S+/.test(e));
  let output = [];
  let opStack = [];
  let inNumber = false;
  let numStack = '';
  for (let i = 0; i < exp.length; i++) {
    if (/\d|\./.test(exp[i])) {
      if (inNumber)
        numStack += exp[i];
      else {
        inNumber = true;
        numStack = exp[i];
      }
    } else if (/\w/.test(exp[i])) {
      if (inNumber) {
        inNumber = false;
        output.push((numStack));
        numStack = '';
      }
      output.push(exp[i]);
    } else if (/\+|-|\*|\/|=|\^/.test(exp[i])) {
      if (inNumber) {
        inNumber = false;
        output.push((numStack));
        numStack = '';
      }
      if (exp[i] == '-') {
        if (i == 0)
          exp[i] = ':';
        else if (exp[i - 1] == '(' || /\+|-|\*|\/|=|\^/.test(exp[i - 1]))
          exp[i] = ':';
      }
      let op = {
        value: exp[i]
      };
      if (exp[i] == ':' || exp[i] == '^')
        op.precedence = 4;
      else if (exp[i] == '*' || exp[i] == '/')
        op.precedence = 3;
      else if (exp[i] == '+' || exp[i] == '-')
        op.precedence = 2;
      else if (exp[i] == '=')
        op.precedence = 1;
      if (typeof opStack.slice(-1)[0] != 'undefined')
        while (opStack.slice(-1)[0].precedence > op.precedence && opStack.slice(-1)[0].value != '(')
          output.push(opStack.pop().value);
      opStack.push(op);
    } else if (exp[i] == '(') {
      if (inNumber) {
        inNumber = false;
        output.push((numStack));
        numStack = '';
      }
      opStack.push({
        value: exp[i],
        precedence: 5
      });
    } else if (exp[i] == ')') {
      if (inNumber) {
        inNumber = false;
        output.push((numStack));
        numStack = '';
      }
      if (typeof opStack.slice(-1)[0] != 'undefined') {
        while (opStack.slice(-1)[0].value != '(')
          output.push(opStack.pop().value);
        if (opStack.slice(-1)[0].value == '(')
          opStack.pop();
      } else throw new SyntaxError('Mismatched parentheses')
    }
  }
  if (numStack.length > 0)
    output.push((numStack));
  while (opStack.length > 0)
    output.push(opStack.pop().value);
  return output;
}
// Reverse Polish notation implemented in JavaScript.
function rpnSolve(exp, data) {
  let stack = [];
  for (let i = 0; i < exp.length; i++) {
    let key = exp[i];
    if (key.match(/\d|[A-Za-z]/))
      stack.push((key));
    let opItems;
    let result;
    switch (key) {
      case '+': // Add.
        opItems = stack.splice(stack.length - 2, 2);
        opItems = opItems.map(item => {
          if (typeof data[item] != 'undefined')
            return parseFloat(data[item]);
          else return parseFloat(item);
        });
        result = opItems[0] + opItems[1];
        stack.push(result);
        break;
      case '-': // Subtract.
        opItems = stack.splice(stack.length - 2, 2);
        opItems = opItems.map(item => {
          if (typeof data[item] != 'undefined')
            return parseFloat(data[item]);
          else return parseFloat(item);
        });
        result = opItems[0] - opItems[1];
        stack.push(result);
        break;
      case '*': // Multiply
        opItems = stack.splice(stack.length - 2, 2);
        opItems = opItems.map(item => {
          if (typeof data[item] != 'undefined')
            return parseFloat(data[item]);
          else return parseFloat(item);
        });
        result = opItems[0] * opItems[1];
        stack.push(result);
        break;
      case '/': // Divide
        opItems = stack.splice(stack.length - 2, 2);
        opItems = opItems.map(item => {
          if (typeof data[item] != 'undefined')
            return parseFloat(data[item]);
          else return parseFloat(item);
        });
        result = opItems[0] / opItems[1];
        stack.push(result);
        break;
      case '^': // Exponentiation
        opItems = stack.splice(stack.length - 2, 2);
        opItems = opItems.map(item => {
          if (typeof data[item] != 'undefined')
            return parseFloat(data[item]);
          else return parseFloat(item);
        });
        result = opItems[0] ** opItems[1];
        stack.push(result);
        break;
      case ':': // Unary negation
        opItems = stack.splice(stack.length - 1, 1);
        opItems = opItems.map(item => {
          if (typeof data[item] != 'undefined')
            return parseFloat(data[item]);
          else return parseFloat(item);
        });
        result = -opItems[0];
        stack.push(result);
        break;
      case '=': // Assignment
        opItems = stack.splice(stack.length - 2, 2);
        data[opItems[0]] = parseFloat(opItems[1]);
        stack.push(parseFloat(opItems[1]));
        break;
    }
  }
  return {
    result: stack[0],
    data: data
  };
};

function solve(exp) {
  let data = {};
  exp = exp.split(';');
  exp.forEach(e => {
    console.log(shuntingYardSolve(e).join(' '));
    let result = rpnSolve(shuntingYardSolve(e), data);
    data = result.data;
    console.log(result);
  });
};

console.log('Math Solver by ElementG9:');
repl('> ', solve);
