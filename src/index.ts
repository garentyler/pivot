import { parse } from './parser.ts';

let result = parse(`
function main() {
  assert(1);
  assert(!0);
}
`);
// console.log(JSON.stringify(result, null, 2));
result.emit();
