import { AST } from './ast.ts';
import { existsSync } from 'https://deno.land/std/fs/mod.ts';

const outputFileName = '../output.s';

const outputToFile = false;
export async function emit(input: string) {
  if (outputToFile) {
    if (existsSync(outputFileName)) {
      await Deno.remove(outputFileName);
    }
    await Deno.create(outputFileName);
    const outputFile = await Deno.open(outputFileName, {
      write: true,
      append: true,
    });
    await Deno.write(outputFile.rid, new TextEncoder().encode(input));
  } else {
    console.log(input);
  }
}

export class Main implements AST {
  constructor(public statements: Array<AST>) {}

  emit() {
    emit('.global main');
    emit('main:');
    emit('  push {fp, lr}');
    this.statements.forEach((statement: any) => statement.emit());
    emit('  mov r0, #0');
    emit('  pop {fp, pc}');
  }
  equals(other: AST): boolean {
    return other instanceof Main
      && this.statements.length === other.statements.length
      && this.statements.every((arg: AST, i: number) => arg.equals(other.statements[i]));
  }
}

export class Assert implements AST {
  constructor(public condition: AST) {}

  emit() {
    this.condition.emit();
    emit('  cmp r0, #1');
    emit(`  moveq r0, #'.'`);
    emit(`  movne r0, #'F'`);
    emit('  bl putchar');
  }
  equals(other: AST): boolean {
    return other instanceof Assert
      && this.condition.equals(other.condition);
  }
}
