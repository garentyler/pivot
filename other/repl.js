const rl = require('readline-sync');
function repl(prompt, func) {
  let answer;
  while (answer != 'exit') {
    answer = rl.question(prompt);
    if (answer == 'exit')
      continue;
    func(answer);
  }
}
module.exports = repl;
