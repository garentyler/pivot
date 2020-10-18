import { AST, Num, Id, Not, Equal, NotEqual, Add, Subtract, Multiply, Divide, FunctionCall, Return, If, While, Var, Assign, Block, FunctionDefinition } from './ast.ts';
import { Main, Assert } from './codegen.ts';

export function parse(source: string): AST {
  // Whitespace and comments.
  let whitespace = Parser.regex(/[ \n\r\t]+/y);
  let comments = Parser.regex(/[/][/].*/y).or(
    Parser.regex(/[/][*].*[*][/]/sy)
  );
  let ignored = Parser.zeroOrMore(whitespace.or(comments));
  // Tokens
  let token = (pattern: RegExp) => Parser.regex(pattern).bind((value: any) => ignored.and(Parser.constant(value)));
  let FUNCTION = token(/function\b/y);
  let IF = token(/if\b/y);
  let ELSE = token(/else\b/y);
  let RETURN = token(/return\b/y);
  let ASSIGN = token(/=/y).map(_ => Assign);
  let VAR = token(/var\b/y);
  let WHILE = token(/while\b/y);
  let COMMA = token(/[,]/y);
  let SEMICOLON = token(/[;]/y);
  let LEFT_PAREN = token(/[(]/y);
  let RIGHT_PAREN = token(/[)]/y);
  let LEFT_BRACE = token(/[{]/y);
  let RIGHT_BRACE = token(/[}]/y);
  let NUMBER = token(/[0-9]/y).map((digits: any) => new Num(parseInt(digits)));
  let ID = token(/[a-zA-Z_][a-zA-Z0-9_]*/y).map((x: any) => new Id(x));
  let NOT = token(/!/y).map(_ => Not);
  let EQUAL = token(/==/y).map(_ => Equal);
  let NOT_EQUAL = token(/!=/y).map(_ => NotEqual);
  let PLUS = token(/[+]/y).map(_ => Add);
  let MINUS = token(/[-]/y).map(_ => Subtract);
  let STAR = token(/[*]/y).map(_ => Multiply);
  let SLASH = token(/[/]/y).map(_ => Divide);
  // Expression parser
  let expression: Parser<AST> = Parser.error('expression parser used before definition');
  // Call parser
  let args: Parser<Array<AST>> = expression.bind((arg: any) => Parser.zeroOrMore(COMMA.and(expression)).bind((args: any) => Parser.constant([arg, ...args]))).or(Parser.constant([]));
  let functionCall: Parser<AST> = ID.bind((callee: any) => LEFT_PAREN.and(args.bind((args: any) => RIGHT_PAREN.and(Parser.constant(callee.equals(new Id('assert')) ? new Assert(args[0]) : new FunctionCall(callee, args))))));
  // Atom
  let atom: Parser<AST> = functionCall.or(ID).or(NUMBER).or(LEFT_PAREN.and(expression).bind((e: any) => RIGHT_PAREN.and(Parser.constant(e))));
  // Unary operators
  let unary: Parser<AST> = Parser.optional(NOT).bind((not: any) => atom.map((operand: any) => not ? new Not(operand) : operand));
  // Infix operators
  let infix = (operatorParser: any, operandParser: any) =>
    operandParser.bind((operand: any) =>
      Parser.zeroOrMore(
        operatorParser.bind((operator: any) =>
          operandParser.bind((operand: any) =>
            Parser.constant({ operator, operand })
          )
        )
      ).map((operatorTerms: any) =>
        operatorTerms.reduce((left: any, { operator, operand }: { operator: any, operand: any }) =>
         new operator(left, operand), operand)
      )
    );
  let product = infix(STAR.or(SLASH), unary);
  let sum = infix(PLUS.or(MINUS), product);
  let comparison = infix(EQUAL.or(NOT_EQUAL), sum);
  // Associativity
  // Closing the loop: expression
  expression.parse = comparison.parse;
  // Statement
  let statement: Parser<AST> = Parser.error('statement parser used before definition');
  let returnStatement: Parser<AST> = RETURN.and(expression).bind((operand: any) => SEMICOLON.and(Parser.constant(new Return(operand))));
  let expressionStatement: Parser<AST> = expression.bind((operand: any) => SEMICOLON.and(Parser.constant(operand)));
  let ifStatement: Parser<AST> = IF.and(LEFT_PAREN).and(expression).bind((conditional: any) =>
    RIGHT_PAREN.and(statement).bind((consequence: any) =>
      ELSE.and(statement).bind((alternative: any) =>
        Parser.constant(new If(conditional, consequence, alternative))
      )
    )
  );
  let whileStatement: Parser<AST> = WHILE.and(LEFT_PAREN).and(expression).bind((conditional: any) =>
    RIGHT_PAREN.and(statement).bind((body: any) =>
      Parser.constant(new While(conditional, body))
    )
  );
  let varStatement: Parser<AST> = VAR.and(ID).bind((name: any) =>
    ASSIGN.and(expression).bind((value: any) =>
      SEMICOLON.and(Parser.constant(new Var(name, value)))
    )
  );
  let assignmentStatement: Parser<AST> = ID.bind((name: any) =>
    ASSIGN.and(expression).bind((value: any) =>
      SEMICOLON.and(Parser.constant(new Assign(name, value)))
    )
  );
  let blockStatement: Parser<AST> = LEFT_BRACE.and(Parser.zeroOrMore(statement)).bind((statements: any) =>
    RIGHT_BRACE.and(Parser.constant(new Block(statements)))
  );
  let parameters: Parser<Array<string>> = ID.bind((param: any) =>
    Parser.zeroOrMore(COMMA.and(ID)).bind((params: any) =>
      Parser.constant([param, ...params])
    )
  ).or(Parser.constant([]));
  let functionStatement: Parser<AST> = FUNCTION.and(ID).bind((name: any) =>
    LEFT_PAREN.and(parameters).bind((parameters: any) =>
      RIGHT_PAREN.and(blockStatement).bind((block: any) =>
        Parser.constant(name.equals(new Id('main')) ? new Main(block.statements) : new FunctionDefinition(name, parameters, block))
      )
    )
  );
  let statementParser: Parser<AST> =
    returnStatement
      .or(functionStatement)
      .or(ifStatement)
      .or(whileStatement)
      .or(varStatement)
      .or(assignmentStatement)
      .or(blockStatement)
      .or(expressionStatement);
  statement.parse = statementParser.parse;
  let parser: Parser<AST> = ignored.and(Parser.zeroOrMore(statement)).map((statements: any) => new Block(statements));

  return parser.parseStringToCompletion(source);
}

export class Parser<T> {
  constructor(public parse: (src: Source) => (ParseResult<T> | null)) {}

  parseStringToCompletion(string: string): T {
    let source = new Source(string, 0);
    let result = this.parse(source);
    if (!result)
      throw Error('Parse error at index 0');
    let index = result.source.index;
    if (index != result.source.string.length)
      throw Error(`Parse error at index ${index}`);
    return result.value;
  }

  static regex(regex: RegExp): Parser<string> {
    return new Parser(source => source.match(regex));
  }
  static constant<U>(value: U): Parser<U> {
    return new Parser(source => new ParseResult(value, source));
  }
  static error<U>(message: string): Parser<U> {
    return new Parser(source => {
      throw Error(message);
    })
  }
  static zeroOrMore<U>(parser: Parser<U>): Parser<Array<U>> {
    return new Parser(source => {
      let results = [];
      let item: any;
      while (item = parser.parse(source)) {
        source = item.source;
        results.push(item.value);
      }
      return new ParseResult(results, source);
    });
  }
  static optional<U>(parser: Parser<U>): Parser<U | null> {
    return parser.or(Parser.constant(null));
  }

  or(parser: Parser<T>): Parser<T> {
    return new Parser(source => {
      let result = this.parse(source);
      return result ? result : parser.parse(source);
    });
  }
  bind<U>(callback: (value: T) => Parser<U>): Parser<U> {
    return new Parser(source => {
      let result = this.parse(source);
      if (result) {
        let value = result.value;
        let source = result.source;
        return callback(value).parse(source);
      } else return null;
    });
  }
  and<U>(parser: Parser<U>): Parser<U> {
    return this.bind(_ => parser);
  }
  map<U>(callback: (t: T) => U): Parser<U> {
    return this.bind(value => Parser.constant(callback(value)))
  }
}
export class Source {
  constructor(public string: string, public index: number) {}

  match(regex: RegExp): (ParseResult<string> | null) {
    regex.lastIndex = this.index;
    let match = this.string.match(regex);
    if (match) {
      let value = match[0];
      let newIndex = this.index + value.length;
      let source = new Source(this.string, newIndex);
      return new ParseResult(value, source);
    }
    return null;
  }
}
export class ParseResult<T> {
  constructor(public value: T, public source: Source) {}
}
