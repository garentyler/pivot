#!/usr/bin/env node
const args = process.argv.slice(2);
const tokenizer = require('../src/tokenizer.js');
const parser = require('../src/parser.js');

if (typeof args[0] != 'undefined') {
  // Execute from file.
} else { // REPL.
  const rl = require('readline-sync');
  const exec = require('child_process').exec;
  function repl(prompt, func) {
    let answer;
    while (answer != 'exit') {
      answer = rl.question(prompt);
      if (answer == 'exit')
        process.exit(0);
      if (answer == 'clear') {
        console.clear();
        continue;
      }
      func(answer);
    }
  }
  console.log('Welcome to Pivot v0.1.0 Alpha.');
  console.log('Type \'exit\' to exit.');
  repl('> ', (answer) => {
    console.log(require('util').inspect(parser.parse(tokenizer.tokenize(answer)), { depth: null }));
  });
}
