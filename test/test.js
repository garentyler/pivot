const assert = require('assert');

const types = require('../src/types.js');
const tokenizer = require('../src/tokenizer.js');
const parser = require('../src/parser.js');

console.log(parser.parse(tokenizer.tokenize('let x = (5 + 3);')));

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
        throw new Error('Changed type: Expected \'' + correct[i].type + '\' but got \'' + t.type + '\'')
      else if (t.subtype != correct[i].subtype)
        throw new Error('Incorrectly changed subtype: Expected \'' + correct[i].subtype + '\' but got \'' + t.subtype + '\'')
      else if (t.value != correct[i].value)
        throw new Error('Changed value: Expected \'' + correct[i].value + '\' but got \'' + t.value + '\'')
    });
  });
  it('getDelimiterToken works', () => {
    let token = tokenizer.util.getDelimiterToken(')');
    if (token.type != 'delimiter')
      throw new Error('Incorrect type: Expected \'delimiter\' but got \'' + token.type + '\'')
    else if (token.subtype != 'right')
      throw new Error('Incorrect subtype: Expected \'right\' but got \'' + token.subtype + '\'')
    else if (token.value != 'parenthesis')
      throw new Error('Incorrect value: Expected \'parenthesis\' but got \'' + token.value + '\'')
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
describe('parser.js', () => {
  it('addIndexes works', () => {
    let tokens = parser.util.addIndexes([{
      type: 'name',
      subtype: 'keyword',
      value: 'let'
    }, {
      type: 'name',
      subtype: 'variable',
      value: 'x'
    }]);
    let correct = [{
      type: 'name',
      subtype: 'keyword',
      value: 'let',
      index: 0
    }, {
      type: 'name',
      subtype: 'variable',
      value: 'x',
      index: 1
    }];
    let isCorrect = true;
    tokens.forEach((t, i) => {
      if (t.type != correct[i].type)
        throw new Error('Changed type: Expected \'' + correct[i].type + '\' but got ' + t.type)
      else if (t.subtype != correct[i].subtype)
        throw new Error('Changed subtype: Expected \'' + correct[i].subtype + '\' but got ' + t.subtype)
      else if (t.value != correct[i].value)
        throw new Error('Changed value: Expected \'' + correct[i].value + '\' but got ' + t.value)
      else if (t.index != correct[i].index)
        throw new Error('Incorrect index: Expected \'' + correct[i].index + '\' but got ' + t.index)
    });
  });
  it('addLevels works', () => {
    let tokens = parser.util.addLevels([{
      type: 'name',
      subtype: 'keyword',
      value: 'let',
      index: 0
    }, {
      type: 'name',
      subtype: 'variable',
      value: 'x',
      index: 1
    }, {
      type: 'operator',
      subtype: 'dual',
      value: '=',
      index: 2
    }, {
      type: 'delimiter',
      subtype: 'left',
      value: 'parenthesis',
      index: 3
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '5',
      index: 4
    }, {
      type: 'delimiter',
      subtype: 'right',
      value: 'parenthesis',
      index: 5
    }]);
    let correct = [{
      type: 'name',
      subtype: 'keyword',
      value: 'let',
      index: 0,
      level: 0
    }, {
      type: 'name',
      subtype: 'variable',
      value: 'x',
      index: 1,
      level: 0
    }, {
      type: 'operator',
      subtype: 'dual',
      value: '=',
      index: 2,
      level: 0
    }, {
      type: 'delimiter',
      subtype: 'left',
      value: 'parenthesis',
      index: 3,
      level: 1
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '5',
      index: 4,
      level: 1
    }, {
      type: 'delimiter',
      subtype: 'right',
      value: 'parenthesis',
      index: 5,
      level: 1
    }];
    let isCorrect = true;
    tokens.forEach((t, i) => {
      if (t.type != correct[i].type)
        throw new Error('Changed type: Expected \'' + correct[i].type + '\' but got ' + t.type)
      else if (t.subtype != correct[i].subtype)
        throw new Error('Changed subtype: Expected \'' + correct[i].subtype + '\' but got \'' + t.subtype + '\'')
      else if (t.value != correct[i].value)
        throw new Error('Changed value: Expected \'' + correct[i].value + '\' but got \'' + t.value + '\'')
      else if (t.index != correct[i].index)
        throw new Error('Incorrect index: Expected \'' + correct[i].index + '\' but got \'' + t.index + '\'')
      else if (t.level != correct[i].level)
        throw new Error('Incorrect level: Expected \'' + correct[i].level + '\' but got \'' + t.level + '\'')
    });
  });
  it('getDeepestLevel works', () => {
    let deepestLevel = parser.util.getDeepestLevel([{
      type: 'name',
      subtype: 'keyword',
      value: 'let',
      index: 0,
      level: 0
    }, {
      type: 'name',
      subtype: 'variable',
      value: 'x',
      index: 1,
      level: 0

    }, {
      type: 'operator',
      subtype: 'dual',
      value: '=',
      index: 2,
      level: 0
    }, {
      type: 'delimiter',
      subtype: 'left',
      value: 'parenthesis',
      index: 3,
      level: 1
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '5',
      index: 4,
      level: 1
    }, {
      type: 'operator',
      subtype: 'dual',
      value: '+',
      index: 5,
      level: 1
    }, {
      type: 'delimiter',
      subtype: 'left',
      value: 'parenthesis',
      index: 6,
      level: 2
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '6',
      index: 7,
      level: 2
    }, {
      type: 'operator',
      subtype: 'dual',
      value: '*',
      index: 8,
      level: 2
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '2',
      index: 9,
      level: 2
    }, {
      type: 'delimiter',
      subtype: 'right',
      value: 'parenthesis',
      index: 10,
      level: 2
    }, {
      type: 'delimiter',
      subtype: 'right',
      value: 'parenthesis',
      index: 11,
      level: 1
    }, {
      type: 'operator',
      subtype: 'none',
      value: ';',
      index: 12,
      level: 0
    }]);
    if (deepestLevel != 2)
      throw new Error('Incorrect deepestLevel. Expected \'2\' but got \'' + deepestLevel + '\'');
  });
  it('combineGroups works', () => {
    let ast = parser.util.combineGroups([{
      type: 'name',
      subtype: 'keyword',
      value: 'let',
      index: 0,
      level: 0
    }, {
      type: 'name',
      subtype: 'variable',
      value: 'x',
      index: 1,
      level: 0

    }, {
      type: 'operator',
      subtype: 'dual',
      value: '=',
      index: 2,
      level: 0
    }, {
      type: 'delimiter',
      subtype: 'left',
      value: 'parenthesis',
      index: 3,
      level: 1
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '5',
      index: 4,
      level: 1
    }, {
      type: 'operator',
      subtype: 'dual',
      value: '+',
      index: 5,
      level: 1
    }, {
      type: 'delimiter',
      subtype: 'left',
      value: 'parenthesis',
      index: 6,
      level: 2
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '6',
      index: 7,
      level: 2
    }, {
      type: 'operator',
      subtype: 'dual',
      value: '*',
      index: 8,
      level: 2
    }, {
      type: 'number',
      subtype: 'n/a',
      value: '2',
      index: 9,
      level: 2
    }, {
      type: 'delimiter',
      subtype: 'right',
      value: 'parenthesis',
      index: 10,
      level: 2
    }, {
      type: 'delimiter',
      subtype: 'right',
      value: 'parenthesis',
      index: 11,
      level: 1
    }, {
      type: 'operator',
      subtype: 'none',
      value: ';',
      index: 12,
      level: 0
    }]);
    if (ast[3].type != 'group')
      throw new Error('Incorrectly combined group.');
    if (ast[3].tokens[3].type != 'group')
      throw new Error('Incorrectly combined group.');
  });
});
