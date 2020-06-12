// This is an unrelated file to Pivot.
// I wanted to put this in the repository in order to test another system of operators.
// This is reverse polish notation, implemented in JavaScript.
var solve = (exp) => {
  var stack = [];
  var expression = exp.split(" ");
  for (var j = 0; j < expression.length; j++) {
    var key = expression[j];
    if (key.match(/\d/)) {
      stack.push(parseInt(key));
    } else if (key.match(/\w/)) {
      if (Object.keys(progData).includes(key)) {
        stack.push(progData[key]);
      } else {
        stack.push(key);
      }
    }
    switch (key) {
      case "+": // add
        var opItems = stack.splice(stack.length - 2, 2);
        var result = opItems[0] + opItems[1];
        stack.push(result);
        break;
      case "-": // subtract
        var opItems = stack.splice(stack.length - 2, 2);
        var result = opItems[0] - opItems[1];
        stack.push(result);
        break;
      case "*": // multiply
        var opItems = stack.splice(stack.length - 2, 2);
        var result = opItems[0] * opItems[1];
        stack.push(result);
        break;
      case "/": // divide
        var opItems = stack.splice(stack.length - 2, 2);
        var result = opItems[0] / opItems[1];
        stack.push(result);
        break;
    }
  }
  return stack;
};
module.exports = solve;
