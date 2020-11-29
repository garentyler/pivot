use crate::ast::AstNode;
use std::ops::Range;
use regex::Regex;

pub fn parse() -> AstNode {
    let src = "420";
    let mut n = String::new();

    println!("src: {:?}", src);
    println!("n: {:?}", n);
    {
        let mut num = Parser::regex(r"\d+(\.\d+)?").bind(&mut n);

        let mut full = num;
        println!("full: {}", full);
        println!("full.parse: {:?}", full.parse(src));
        println!("full: {}", full);
    }
    println!("src: {:?}", src);
    println!("n: {:?}", n);
    AstNode::block(vec![])
}

#[derive(Debug)]
pub enum ParserKind<'a> {
    Literal(String),
    Regex(Regex),
    And,
    Or,
    Repeat(usize),
    RepeatRange(Range<usize>),
    Bind(&'a mut String),
}
impl<'a> std::fmt::Display for ParserKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserKind::*;
        match self {
            Literal(s) => write!(f, "Literal \"{}\"", s),
            Regex(r) => write!(f, "Regex /{}/", r.as_str()),
            And => write!(f, "And"),
            Or => write!(f, "Or"),
            Repeat(num) => write!(f, "Repeat {}", num),
            RepeatRange(range) => write!(f, "RepeatRange {:?}", range),
            Bind(_) => write!(f, "Bind"),
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    kind: ParserKind<'a>,
    subparsers: Vec<Parser<'a>>,
    // bind: Option<&'a mut String>,
}
impl<'a> std::fmt::Display for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}
impl<'a> Parser<'a> {
    pub fn parse<T: Into<String>>(&mut self, src: T) -> Result<(String, String), String> {
        use ParserKind::*;
        let s: String = src.into();
        match &mut self.kind {
            Literal(literal) => {
                if s.len() >= literal.len() && s[..literal.len()] == literal[..] {
                    Ok((s[..literal.len()].to_owned(), s[literal.len()..].to_owned()))
                } else {
                    Err(s)
                }
            }
            Regex(re) => {
                if let Some(mat) = re.find(&s) {
                    if mat.start() == 0 {
                        Ok((s[mat.start()..mat.end()].to_owned(), s[mat.end()..].to_owned()))
                    } else {
                        Err(s)
                    }
                } else {
                    Err(s)
                }
            }
            And => {
                let (lmatched, lrest) = self.subparsers[0].parse(s)?;
                let (rmatched, rrest) = self.subparsers[1].parse(lrest)?;
                Ok((lmatched + &rmatched, rrest))
            }
            Or => {
                if let Ok(lresult) = self.subparsers[0].parse(s.clone()) {
                    Ok(lresult)
                } else {
                    self.subparsers[1].parse(s.clone())
                }
            }
            Repeat(num_repeats) => {
                let mut matched = String::new();
                let mut rest = s.clone();
                for _ in 0..*num_repeats {
                    let (m, r) = self.subparsers[0].parse(rest)?;
                    matched += &m;
                    rest = r;
                }
                Ok((matched, rest))
            }
            RepeatRange(range) => {
                let mut matched = String::new();
                let mut rest = s.clone();

                // Parse up to range.start
                for _ in 0..range.start {
                    let (m, r) = self.subparsers[0].parse(rest)?;
                    matched += &m;
                    rest = r;
                }

                // Parse optionally up to range.end
                for _ in 0..(range.end - range.start) {
                    let parse_result = self.subparsers[0].parse(rest);
                    if let Err(r) = parse_result {
                        rest = r;
                        break;
                    } else {
                        let (m, r) = parse_result.unwrap();
                        matched += &m;
                        rest = r;
                    }
                }

                Ok((matched, rest))
            }
            Bind(var) => {
                let (matched, rest) = self.subparsers[0].parse(s)?;
                **var = matched.clone();
                Ok((matched, rest))
            }
        }
    }

    // Static
    pub fn literal<T: Into<String>>(s: T) -> Parser<'a> {
        Parser {
            kind: ParserKind::Literal(s.into()),
            subparsers: vec![],
            // bind: None,
        }
    }
    pub fn regex<T: Into<String>>(s: T) -> Parser<'a> {
        Parser {
            kind: ParserKind::Regex(Regex::new(&s.into()).expect("could not compile regex")),
            subparsers: vec![],
            // bind: None,
        }
    }

    // Instance
    pub fn and(self, r: Parser<'a>) -> Parser<'a> {
        Parser {
            kind: ParserKind::And,
            subparsers: vec![self, r],
            // bind: None,
        }
    }
    pub fn or(self, r: Parser<'a>) -> Parser<'a> {
        Parser {
            kind: ParserKind::Or,
            subparsers: vec![self, r],
            // bind: None,
        }
    }
    pub fn repeat(self, num_repeats: usize) -> Parser<'a> {
        Parser {
            kind: ParserKind::Repeat(num_repeats),
            subparsers: vec![self],
            // bind: None,
        }
    }
    pub fn repeat_range(self, num_repeats: Range<usize>) -> Parser<'a> {
        Parser {
            kind: ParserKind::RepeatRange(num_repeats),
            subparsers: vec![self],
            // bind: None,
        }
    }
    pub fn optional(self) -> Parser<'a> {
        Parser {
            kind: ParserKind::RepeatRange(0..1),
            subparsers: vec![self],
            // bind: None,
        }
    }
    pub fn bind(self, s: &'a mut String) -> Parser<'a> {
        Parser {
            kind: ParserKind::Bind(s),
            subparsers: vec![self],
        }
    }

    // Other
    pub fn pretty_print(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        for _ in 0..indent {
            write!(f, " ");
        }
        write!(f, "{}", self.kind)?;
        if self.subparsers.len() > 0 {
            write!(f, " [\n")?;
            for subparser in &self.subparsers {
                subparser.pretty_print(f, indent + 2)?;
                write!(f, ",\n")?;
            }
            for _ in 0..indent {
                write!(f, " ")?;
            }
            write!(f, "]")
        } else {
            write!(f, "")
        }
    }
}

// use combinators::*;
// pub mod combinators {
//     pub struct Parser<'a> {
//         source: &'a str,
//         subparsers: Vec<Parser<'a>>,
//         pub parse: Box<Fn(&'a str) -> Result<(&'a str, &'a str), &'a str>>,
//     }
//     impl Parser {
//         // pub type S = Into<String>;
//         pub fn literal<'a>(literal: &'a str) -> Parser {
//             Parser {
//                 source: literal.into(),
//                 subparsers: vec![],
//                 parse: Box::new(|s: &'a str| -> Result<(&'a str, &'a str), &'a str> {
//                     if src.len() >= literal.len() {
//                         if src[..literal.len()] == literal[..] {
//                             return Ok((&src[..literal.len()], &src[literal.len()..]));
//                         }
//                     }
//                     Err(&src[..])
//                 })
//             }
//         }
//     }
    // pub fn literal<'a, T>(literal: &'a str) -> T where T: Fn(&str) -> Result<(&str, &str), &str> + 'a {
    //     move |src: &str| -> Result<(&str, &str), &str> {
    //         if src.len() >= literal.len() {
    //             if src[..literal.len()] == literal[..] {
    //                 return Ok((&src[..literal.len()], &src[literal.len()..]));
    //             }
    //         }
    //         Err(&src[..])
    //     }
    // }
    // pub fn and<'a>(left: T, right: T) -> T where T: Fn(&str) -> Result<(&str, &str), &str> + 'a {
    //
    // }
// }

// // Whitespace and comments.
// let whitespace = Parser.regex(/[ \n\r\t]+/y);
// let comments = Parser.regex(/[/][/].*/y).or(
//   Parser.regex(/[/][*].*[*][/]/sy)
// );
// let ignored = Parser.zeroOrMore(whitespace.or(comments));
// // Tokens
// let token = (pattern: RegExp) => Parser.regex(pattern).bind((value: any) => ignored.and(Parser.constant(value)));
// let FUNCTION = token(/function\b/y);
// let IF = token(/if\b/y);
// let ELSE = token(/else\b/y);
// let RETURN = token(/return\b/y);
// let ASSIGN = token(/=/y).map(_ => Assign);
// let VAR = token(/var\b/y);
// let WHILE = token(/while\b/y);
// let COMMA = token(/[,]/y);
// let SEMICOLON = token(/[;]/y);
// let LEFT_PAREN = token(/[(]/y);
// let RIGHT_PAREN = token(/[)]/y);
// let LEFT_BRACE = token(/[{]/y);
// let RIGHT_BRACE = token(/[}]/y);
// let NUMBER = token(/[0-9]/y).map((digits: any) => new Num(parseInt(digits)));
// let ID = token(/[a-zA-Z_][a-zA-Z0-9_]*/y).map((x: any) => new Id(x));
// let NOT = token(/!/y).map(_ => Not);
// let EQUAL = token(/==/y).map(_ => Equal);
// let NOT_EQUAL = token(/!=/y).map(_ => NotEqual);
// let PLUS = token(/[+]/y).map(_ => Add);
// let MINUS = token(/[-]/y).map(_ => Subtract);
// let STAR = token(/[*]/y).map(_ => Multiply);
// let SLASH = token(/[/]/y).map(_ => Divide);
// // Expression parser
// let expression: Parser<AST> = Parser.error('expression parser used before definition');
// // Call parser
// let args: Parser<Array<AST>> = expression.bind((arg: any) => Parser.zeroOrMore(COMMA.and(expression)).bind((args: any) => Parser.constant([arg, ...args]))).or(Parser.constant([]));
// let functionCall: Parser<AST> = ID.bind((callee: any) => LEFT_PAREN.and(args.bind((args: any) => RIGHT_PAREN.and(Parser.constant(callee.equals(new Id('assert')) ? new Assert(args[0]) : new FunctionCall(callee, args))))));
// // Atom
// let atom: Parser<AST> = functionCall.or(ID).or(NUMBER).or(LEFT_PAREN.and(expression).bind((e: any) => RIGHT_PAREN.and(Parser.constant(e))));
// // Unary operators
// let unary: Parser<AST> = Parser.optional(NOT).bind((not: any) => atom.map((operand: any) => not ? new Not(operand) : operand));
// // Infix operators
// let infix = (operatorParser: any, operandParser: any) =>
//   operandParser.bind((operand: any) =>
//     Parser.zeroOrMore(
//       operatorParser.bind((operator: any) =>
//         operandParser.bind((operand: any) =>
//           Parser.constant({ operator, operand })
//         )
//       )
//     ).map((operatorTerms: any) =>
//       operatorTerms.reduce((left: any, { operator, operand }: { operator: any, operand: any }) =>
//        new operator(left, operand), operand)
//     )
//   );
// let product = infix(STAR.or(SLASH), unary);
// let sum = infix(PLUS.or(MINUS), product);
// let comparison = infix(EQUAL.or(NOT_EQUAL), sum);
// // Associativity
// // Closing the loop: expression
// expression.parse = comparison.parse;
// // Statement
// let statement: Parser<AST> = Parser.error('statement parser used before definition');
// let returnStatement: Parser<AST> = RETURN.and(expression).bind((operand: any) => SEMICOLON.and(Parser.constant(new Return(operand))));
// let expressionStatement: Parser<AST> = expression.bind((operand: any) => SEMICOLON.and(Parser.constant(operand)));
// let ifStatement: Parser<AST> = IF.and(LEFT_PAREN).and(expression).bind((conditional: any) =>
//   RIGHT_PAREN.and(statement).bind((consequence: any) =>
//     ELSE.and(statement).bind((alternative: any) =>
//       Parser.constant(new If(conditional, consequence, alternative))
//     )
//   )
// );
// let whileStatement: Parser<AST> = WHILE.and(LEFT_PAREN).and(expression).bind((conditional: any) =>
//   RIGHT_PAREN.and(statement).bind((body: any) =>
//     Parser.constant(new While(conditional, body))
//   )
// );
// let varStatement: Parser<AST> = VAR.and(ID).bind((name: any) =>
//   ASSIGN.and(expression).bind((value: any) =>
//     SEMICOLON.and(Parser.constant(new Var(name, value)))
//   )
// );
// let assignmentStatement: Parser<AST> = ID.bind((name: any) =>
//   ASSIGN.and(expression).bind((value: any) =>
//     SEMICOLON.and(Parser.constant(new Assign(name, value)))
//   )
// );
// let blockStatement: Parser<AST> = LEFT_BRACE.and(Parser.zeroOrMore(statement)).bind((statements: any) =>
//   RIGHT_BRACE.and(Parser.constant(new Block(statements)))
// );
// let parameters: Parser<Array<string>> = ID.bind((param: any) =>
//   Parser.zeroOrMore(COMMA.and(ID)).bind((params: any) =>
//     Parser.constant([param, ...params])
//   )
// ).or(Parser.constant([]));
// let functionStatement: Parser<AST> = FUNCTION.and(ID).bind((name: any) =>
//   LEFT_PAREN.and(parameters).bind((parameters: any) =>
//     RIGHT_PAREN.and(blockStatement).bind((block: any) =>
//       Parser.constant(name.equals(new Id('main')) ? new Main(block.statements) : new FunctionDefinition(name, parameters, block))
//     )
//   )
// );
// let statementParser: Parser<AST> =
//   returnStatement
//     .or(functionStatement)
//     .or(ifStatement)
//     .or(whileStatement)
//     .or(varStatement)
//     .or(assignmentStatement)
//     .or(blockStatement)
//     .or(expressionStatement);
// statement.parse = statementParser.parse;
// let parser: Parser<AST> = ignored.and(Parser.zeroOrMore(statement)).map((statements: any) => new Block(statements));
//
// return parser.parseStringToCompletion(source);
