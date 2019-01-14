// Import the token class.
const token = require("./classes.js").token;

// Create the tokenizer function.
// tokenize() takes Pivot code in, and outputs an array of tokens.
module.exports =  exp => {
	// Check functions for different token types.
	var isDigit = char => {
		return /\d/.test(char);
	};
	var isLetter = char => {
		return /[a-z]/i.test(char);
	};
	var isOperator = char => {
		return /\+|-|\*|\/|\^|=/.test(char);
	};
	var isLeftDelimiter = char => {
		return (/\(|\[|\{|"|'|`/.test(char));
	};
	var isRightDelimiter = char => {
		return (/\)|\]|\}/.test(char));
	};
	var isComma = char => {
		return (char === ",");
	};
	var isPeriod = char => {
		return (char === ".");
	};
	var result = []; // The final array of tokens.
	var nb = []; // Number buffer. Allows for multiple digits to be one number.
	var lb = []; // Letter buffer. Allows for multiple letters to be one variable / function.
	var ob = []; // Operator buffer. Allows for multi-character operators. E.g. ++ or ==.
	var sb = []; // String buffer. Allows for multi-character strings.
	var inString = false; // Keep track of whether in string or not.
	var stringType; // Keep track of what type of string. E.g. "" or ''.
	exp = exp.split(""); // Split the expression into an array of characters.
	/* - - - DO NOT TOUCH THIS - - - */
	for (var i = 0; i < exp.length; i++) { // Loop through all of the characters.
		var char = exp[i]; // Create a quick reference to the current char.
		if (i >= 1) {
			if (exp[i - 1] == "\\") {
				exp.splice(i - 1, 2, `\\${char}`);
				i--;
				continue;
			}
			if (exp[i - 1] == "$" && char == "{") {
				exp.splice(i - 1, 2, `\${`);
				i--;
				continue;
			}
		}
	}
	/* - - - OK YOU CAN TOUCH AGAIN - - - */
	// Nevermind, just don't mess with any of this file.
	for (var i = 0; i < exp.length; i++) {
		var char = exp[i];
		if (inString) {
			if (char == `'` || char == `"` || char == "`") {
				var exitString = () => {
					inString = false;
					if (sb.length == 0) {
						result.push(new token("String", null));
					} else {
						var string = sb.join("");
						result.push(new token("String", string));
					}
					sb = [];
				};
				if (char == `'` && stringType == "single") {
					exitString();
				} else if (char == `"` && stringType == "double") {
					exitString();
				} else if (char == "`" && stringType == "backtick") {
					exitString();
				} else {
					if (char == `'`) {
						sb.push(`\'`);
					}
					if (char == `"`) {
						sb.push(`\"`);
					}
					if (char == "`") {
						sb.push("\`");
					}
				}
			} else {
				sb.push(char);
			}
		} else {
			if (isDigit(char)) {
				result.push(new token("Operator", ob.join("")));
				ob = [];
				nb.push(char);
			} else if (isLetter(char)) {
				result.push(new token("Operator", ob.join("")));
				ob = [];
				lb.push(char);
			} else if (isOperator(char)) {
				result.push(new token("Number", nb.join("")));
				nb = [];
				result.push(new token("Variable", lb.join("")));
				lb = [];
				ob.push(char);
			} else if (isLeftDelimiter(char)) {
				result.push(new token("Operator", ob.join("")));
				ob = [];
				result.push(new token("Function Call", lb.join("")));
				lb = [];
				if (char == `'` || char == `"` || char == "`") {
					inString = true;
					if (char == `'`) {
						stringType = "single";
					} else if (char == `"`) {
						stringType = "double";
					} else if (char == "`") {
						stringType = "backtick";
					}
				} else {
					result.push(new token("Left Delimiter", char));
				}
			} else if (isRightDelimiter(char)) {
				result.push(new token("Operator", ob.join("")));
				ob = [];
				result.push(new token("Number", nb.join("")));
				nb = [];
				result.push(new token("Variable", lb.join("")));
				lb = [];
				result.push(new token("Right Delimiter", char));
			} else if (isComma(char)) {
				result.push(new token("Operator", ob.join("")));
				ob = [];
				result.push(new token("Number", nb.join("")));
				nb = [];
				result.push(new token("Variable", lb.join("")));
				lb = [];
				result.push(new token("Comma", char));
			} else if (isPeriod(char)) {
				result.push(new token("Operator", ob.join("")));
				ob = [];
				nb.push(char);
			}
		}
	}
	result.push(new token("Operator", ob.join("")));
	ob = [];
	result.push(new token("Number", nb.join("")));
	nb = [];
	lb.forEach(item => {
		result.push(new token("Variable", item));
	});
	lb = [];
	for (var i = 0; i < 3; i++) {
		result.forEach((item, index) => {
			if (item.value == "") {
				result.splice(index, 1);
			}
		});
	}
	result.forEach((item, index) => {
		if (item.value == "-" && index != 0) {
			if (result[index - 1].type != "Variable" && result[index - 1].type != "Number") {
				if (result[index + 1].type == "Number") {
					result[index + 1].value = "-" + result[index + 1].value;
					result.splice(index, 1);
				}
			}
		}
	});
	return result;
};
