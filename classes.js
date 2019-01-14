// This is the most basic type of token.
var token = function (type, value) {
	this.type = type;
	this.value = value;
};
// This is a group of tokens.
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
module.exports = {
	token: token,
	group: group
}
