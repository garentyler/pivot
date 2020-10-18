import { emit } from './codegen.ts';

export interface AST {
  emit(): void;
  equals(other: AST): boolean;
}

// Number node.
export class Num implements AST {
  constructor(public value: number) {}

  emit() {
    emit(`  ldr r0, =${this.value}`);
  }
  equals(other: AST): boolean {
    return other instanceof Num
      && this.value === other.value;
  }
}
// Identifier node.
export class Id implements AST {
  constructor(public value: string) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof Id
      && this.value === other.value;
  }
}
// Operator nodes.
export class Not implements AST {
  constructor(public operand: AST) {}

  emit() {
    this.operand.emit();
    emit('  cmp r0, #0');
    emit('  moveq r0, #1');
    emit('  movne r0, #0');
  }
  equals(other: AST): boolean {
    return other instanceof Not
      && this.operand.equals(other.operand);
  }
}
export abstract class Infix {
  constructor(public left: AST, public right: AST) {}
  emit() {
    this.left.emit();
    emit('  push {r0, ip}');
    this.right.emit();
    emit('  pop {r1, ip}');
  }
}
export class Equal extends Infix implements AST {
  constructor(public left: AST, public right: AST) {
    super(left, right);
  }

  emit() {
    super.emit();
    emit('  cmp r0, r1');
    emit('  moveq r0, #1');
    emit('  movne r0, #0');
  }
  equals(other: AST): boolean {
    return other instanceof Equal
      && this.left.equals(other.left)
      && this.right.equals(other.right);
  }
}
export class NotEqual extends Infix implements AST {
  constructor(public left: AST, public right: AST) {
    super(left, right);
  }

  emit() {
    super.emit();
    emit('  cmp r0, r1');
    emit('  moveq r0, #0');
    emit('  movne r0, #1');
  }
  equals(other: AST): boolean {
    return other instanceof NotEqual
      && this.left.equals(other.left)
      && this.right.equals(other.right);
  }
}
export class Add extends Infix implements AST {
  constructor(public left: AST, public right: AST) {
    super(left, right);
  }

  emit() {
    super.emit();
    emit('  add r0, r0, r1');
  }
  equals(other: AST): boolean {
    return other instanceof Add
      && this.left.equals(other.left)
      && this.right.equals(other.right);
  }
}
export class Subtract extends Infix implements AST {
  constructor(public left: AST, public right: AST) {
    super(left, right);
  }

  emit() {
    super.emit();
    emit('  sub r0, r0, r1');
  }
  equals(other: AST): boolean {
    return other instanceof Subtract
      && this.left.equals(other.left)
      && this.right.equals(other.right);
  }
}
export class Multiply extends Infix implements AST {
  constructor(public left: AST, public right: AST) {
    super(left, right);
  }

  emit() {
    super.emit();
    emit('  mul r0, r0, r1');
  }
  equals(other: AST): boolean {
    return other instanceof Multiply
      && this.left.equals(other.left)
      && this.right.equals(other.right);
  }
}
export class Divide extends Infix implements AST {
  constructor(public left: AST, public right: AST) {
    super(left, right);
  }

  emit() {
    super.emit();
    emit('  udiv r0, r0, r1');
  }
  equals(other: AST): boolean {
    return other instanceof Divide
      && this.left.equals(other.left)
      && this.right.equals(other.right);
  }
}
// Function call node.
export class FunctionCall implements AST {
  constructor(public callee: string, public args: Array<AST>) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof FunctionCall
      && this.callee === other.callee
      && this.args.length === other.args.length
      && this.args.every((arg: AST, i: number) => arg.equals(other.args[i]));
  }
}
// Return node.
export class Return implements AST {
  constructor(public operand: AST) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof Return
      && this.operand.equals(other.operand);
  }
}
// Block node.
export class Block implements AST {
  constructor(public statements: Array<AST>) {}

  emit() {
    this.statements.forEach((statement: any) => statement.emit());
  }
  equals(other: AST): boolean {
    return other instanceof Block
      && this.statements.length === other.statements.length
      && this.statements.every((arg: AST, i: number) => arg.equals(other.statements[i]));
  }
}
// If node.
export class If implements AST {
  constructor(public conditional: AST,
              public consequence: AST,
              public alternative: AST) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof If
      && this.conditional.equals(other.conditional)
      && this.consequence.equals(other.consequence)
      && this.alternative.equals(other.alternative);
  }
}
// Function definition node.
export class FunctionDefinition implements AST {
  constructor(public name: string,
              public parameters: Array<string>,
              public body: AST) {}

  emit() {
    throw Error(`Not implemented yet (${this.name})`);
  }
  equals(other: AST): boolean {
    return other instanceof FunctionDefinition
      && this.name === other.name
      && this.parameters.length === other.parameters.length
      && this.parameters.every((arg: string, i: number) => arg === other.parameters[i])
      && this.body.equals(other.body);
  }
}
// Variable declaration node.
export class VariableDeclaration implements AST {
  constructor(public name: string, public value: AST) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof VariableDeclaration
      && this.name === other.name
      && this.value.equals(other.value);
  }
}
// Assignment node.
export class Assign implements AST {
  constructor(public name: string, public value: AST) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof Assign
      && this.name === other.name
      && this.value.equals(other.value);
  }
}
// While loop node.
export class While implements AST {
  constructor(public conditional: AST, public body: AST) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof While
      && this.conditional.equals(other.conditional)
      && this.body.equals(other.body);
  }
}
// Variable node.
export class Var implements AST {
  constructor(public name: string, public value: AST) {}

  emit() {
    throw Error('Not implemented yet');
  }
  equals(other: AST): boolean {
    return other instanceof Var
      && this.name === other.name
      && this.value.equals(other.value);
  }
}
