const assert = require('assert');

const types = require('../src/types.js');
const tokenizer = require('../src/tokenizer.js');
const parser = require('../src/parser.js');

describe('types.js', () => {
  it('Has a Token child.', () => {
    assert.equal(types.hasOwnProperty('Token'), true);
  });
  it('Has a Group child.', () => {
    assert.equal(types.hasOwnProperty('Group'), true);
  });

  describe('Token', () => {
    it('Works as a constructor', () => {
      try {
        let token = new types.Token('a', 'b', 'c');
      } catch (err) {
        throw err;
      }
    });
    it('Has values \'type\', \'subtype\', and \'value\'', () => {
      try {
        let token = new types.Token('a', 'b', 'c');
        if (!token.hasOwnProperty('type') || !token.hasOwnProperty('subtype') || !token.hasOwnProperty('value'))
          throw new Error('Token is missing \'type\', \'subtype\', or \'value\' properties.');
        if (token.type != 'a' || token.subtype != 'b' || token.value != 'c')
          throw new Error('Token incorrectly set \'type\', \'subtype\', or \'value\' properties.');
      } catch (err) {
        throw err;
      }
    });
  });
  describe('Group', () => {
    it('Works as a constructor', () => {
      try {
        let group = new types.Group('a', 'b');
      } catch (err) {
        throw err;
      }
    });
    it('Has values \'type\', \'subtype\', and \'tokens\'', () => {
      try {
        let group = new types.Group('a', 'b');
        if (!group.hasOwnProperty('type') || !group.hasOwnProperty('subtype') || !group.hasOwnProperty('tokens'))
          throw new Error('Group is missing \'type\', \'subtype\', or \'tokens\' properties.');
        if (group.type != 'group' || group.subtype != 'a' || group.tokens != 'b')
          throw new Error('Group incorrectly set \'type\', \'subtype\', or \'tokens\' properties.');
      } catch (err) {
        throw err;
      }
    });
  });
});
describe('tokenizer.js', () => {
  it('Has a tokenize child', () => {
    assert.equal(tokenizer.hasOwnProperty('tokenize'), true);
  });
  it('Has a util child', () => {
    assert.equal(tokenizer.hasOwnProperty('util'), true);
  });
  describe('util', () => {
    it('combineEscapedChars works', () => {
      assert.equal(tokenizer.util.combineEscapedChars(`let x = 'test\\nnewline';`.split('')).join(''), `let x = 'test\\nnewline';`);
    });
    it('removeComments works', () => {
      assert.equal(tokenizer.util.removeComments(`// Comment\nlet i = 0;`.split('')).join(''), `let i = 0;`);
    });
    it('changeKeywords works', () => {
      let tokens = tokenizer.util.changeKeywords([{
        type: 'name',
        subtype: 'variable',
        value: 'let'
      }, {
        type: 'name',
        subtype: 'variable',
        value: 'x'
      }]);
      let correct = [{
        type: 'name',
        subtype: 'keyword',
        value: 'let'
      }, {
        type: 'name',
        subtype: 'variable',
        value: 'x'
      }];
      let isCorrect = true;
      tokens.forEach((t, i) => {
        if (t.type != correct[i].type)
          throw new Error('Changed type: Expected \''+ correct[i].type +'\' but got ' + t.type)
        else if (t.subtype != correct[i].subtype)
          throw new Error('Incorrectly changed subtype: Expected \''+ correct[i].subtype +'\' but got ' + t.subtype)
        else if (t.value != correct[i].value)
          throw new Error('Changed value: Expected \''+ correct[i].value +'\' but got ' + t.value)
      });
    });
    it('getDelimiterToken works', () => {
      let token = tokenizer.util.getDelimiterToken(')');
      if (token.type != 'delimiter')
        throw new Error('Incorrect type: Expected \'delimiter\' but got ' + token.type)
      else if (token.subtype != 'right')
        throw new Error('Incorrect subtype: Expected \'right\' but got ' + token.subtype)
      else if (token.value != 'parenthesis')
        throw new Error('Incorrect value: Expected \'parenthesis\' but got ' + token.value)
    });
    it('operatorType works', () => {
      assert.equal(tokenizer.util.operatorType('++'), 'left');
      assert.equal(tokenizer.util.operatorType(';'), 'none');
      assert.equal(tokenizer.util.operatorType('+'), 'dual');
    });
    it('determineCharType works', () => {
      assert.equal(tokenizer.util.determineCharType('+'), 'operator');
      assert.equal(tokenizer.util.determineCharType('"'), 'string delimiter');
      assert.equal(tokenizer.util.determineCharType('4'), 'digit');
    });
    it('determineType works', () => {
      assert.equal(tokenizer.util.determineType('let'), 'keyword');
      assert.equal(tokenizer.util.determineType('dog'), 'unknown');
    });
  });
});
describe('parser.js', () => {
  
});
