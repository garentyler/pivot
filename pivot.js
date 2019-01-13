var token = function (type, value) {
	this.type = type;
	this.value = value;
};
var group = function (type, tokens) {
    this.type = "Group";
    this.value = type;
    this.tokens;
    if(typeof tokens != "undefined") {
        this.tokens = tokens;
    } else {
        this.tokens = [];
    }
}
var tokenize = exp => {
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
	var result = [];
	var nb = [];
	var lb = [];
	var ob = [];
	var sb = [];
	var inString = false;
	var stringType;
	exp = exp.split("");
	for (var i = 0; i < exp.length; i++) {
		var char = exp[i];
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
var parse = tokens => {
	var layer = 0;
    var delimiterCount = 0;
    var deepestLayer = 0;
	for (var i = 0; i < tokens.length; i++) {
		if (tokens[i].type == "Left Delimiter") {
            layer++;
            if(layer > deepestLayer) {
                deepestLayer = layer;
            }
			delimiterCount++;
		}
		tokens[i].layer = layer;
		if (tokens[i].type == "Right Delimiter") {
			layer--;
		}
    }
	for (var i = 0; i < tokens.length; i++) {
		if ((tokens[i].type == "Left Delimiter") || (tokens[i].type == "Right Delimiter")) {
            tokens[i].layer--;
		}
	}
	if (layer > 0) { // Unclosed delimiter
	} else if (layer < 0) { // Overclosed delimiter
	}
    layer = 0;
    for(var i=deepestLayer;i>=0;i--) {
        var temp = [];
        var firstIndex;
        for(var j=0;j<tokens.length;j++) {
            console.log(i);
            console.log(temp);
            console.log(tokens);
            console.log();
            if(tokens[j].layer == i) {
                if(temp.length <= 0) {
                    firstIndex = j;
                }
                temp.push(tokens[j]);
            } else {
                if(temp.length > 0) {
                    var g = new group(tokens[firstIndex].value,temp);
                    tokens.splice(firstIndex-1,temp.length+2,g);
                    temp = [];
                }
            }
        }
    }
	return tokens;
};
var fs = require("fs");
var ast = parse(tokenize(`asdf = (a) {return(a++)}`));
fs.writeFileSync("ast.json", JSON.stringify(ast, null, 4));