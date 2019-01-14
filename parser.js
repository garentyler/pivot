// Import the group class.
const group = require("./classes.js").group;

// Create the parser function.
// parse() takes an array of tokens in, and outputs an
// Abstract Syntax Tree (a structured array of tokens).
module.exports = tokens => {
	// Variables for later.
	var layer = 0;
    var delimiterCount = 0;
    var deepestLayer = 0;
	// Give each token a layer number based on delimiters.
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
	// Lower the layer of delimiters.
	for (var i = 0; i < tokens.length; i++) {
		if ((tokens[i].type == "Left Delimiter") || (tokens[i].type == "Right Delimiter")) {
            tokens[i].layer--;
		}
	}
	if (layer > 0) { // Unclosed delimiter.
	} else if (layer < 0) { // Overclosed delimiter.
	}
	// Reset layer for structuring.
    layer = 0;
	/* - - - DO NOT TOUCH THIS - - - */
    for(var i=deepestLayer;i>=0;i--) {
        var temp = [];
        var firstIndex;
        for(var j=0;j<tokens.length;j++) {
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
	/* - - - OK YOU CAN TOUCH AGAIN - - - */
	// Return the structured tokens.
	return tokens;
};
