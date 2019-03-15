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
      if (layer > deepestLayer) {
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
  // Give each token an index.
  for (let i = 0; i < tokens.length; i++) {
    tokens[i].index = i;
  }
  // Structure the layers.
  // Count the rising edges of the layers to determine how many groups should exist.
  let structure = function () {
    let layer = 0;
    let risingFalling = []; // Create an array to store indices of rising/falling edges.
    for (let i = 0; i < tokens.length; i++) {
      // Add a rising and a falling tag to each token.
      tokens[i].rising = false;
      tokens[i].falling = false;
      if (tokens[i].layer > layer) { // If the token moves up a layer.
        // Create a new rising index in risingFalling.
        risingFalling.push({
          type: 'rising',
          index: i
        });
        tokens[i].rising = true; // Note that the token is a rising edge.
        layer++;
      } else if (tokens[i].layer < layer) {
        // Create a new falling index in risingFalling.
        risingFalling.push({
          type: 'falling',
          index: i
        });
        tokens[i].falling = true; // Note that the token is a falling edge.
        layer--;
      }
    }
    // Loop through the list of rising/falling edges.
    for (let i = 0; i < risingFalling.length; i++) {
      if (i != risingFalling.length - 1) { // If not the last edge.
        let item = risingFalling[i];
        let nextItem = risingFalling[i + 1];
        // If a falling edge follows a rising edge, classifiy it as a group.
        if ((item.type == 'rising') && (nextItem.type == 'falling')) {
          // Get the group together as one item.
          let selectedItems = tokens.slice(item.index, nextItem.index);
          tokens.splice(item.index, selectedItems.length, new group(tokens[item.index - 1].value, selectedItems, item.index));
        }
      }
    }
    risingFalling = []; // Reset the list of edges.
    // Count the edges again.
    for (let i = 0; i < tokens.length; i++) {
      if (tokens[i].layer > layer) {
        risingFalling.push({
          type: 'rising',
          index: i
        });
        layer++;
      } else if (tokens[i].layer < layer) {
        risingFalling.push({
          type: 'falling',
          index: i
        });
        layer--;
      }
    }
    // If there are still edges, run again.
    if (risingFalling.length) {
      structure();
    }
  };
  // Start the recursion.
  structure();
  let trimDelimiters = function (thing) {
    // Loop through the tokens of thing.
    for (let i = 0; i < thing.length; i++) {
      // Delete unnecessary keys.
      if (typeof thing[i].rising != 'undefined') {
        delete thing[i].rising;
      }
      if (typeof thing[i].falling != 'undefined') {
        delete thing[i].falling;
      }
      if (typeof thing[i].index != 'undefined') {
        delete thing[i].index;
      }
      if (typeof thing[i].index != 'undefined') {
        delete thing[i].index;
      }
      // Remove delimiters.
      if ((thing[i].type == 'Left Delimiter') || (thing[i].type == 'Right Delimiter')) {
        thing.splice(i, 1);
        i--;
      }
      // If a token is a group, look at the group's tokens.
      if (thing[i].type == 'Group') {
        trimDelimiters(thing[i].tokens);
      }
    }
  };
  // Start the recursion.
  trimDelimiters(tokens);
  // Return the structured tokens.
  return tokens;
};