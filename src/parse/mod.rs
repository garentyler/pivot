mod combinators;

use crate::ast::AstNode;
use combinators::Parser;
use ron::{from_str, to_string};

pub fn parse<T: Into<String>>(src: T) -> AstNode {
    let src: String = src.into();
    from_str::<AstNode>(&parse_expression(src).unwrap().0).unwrap()
}
fn parse_expression(src: String) -> Result<(String, String), String> {
    let whitespace = Parser::regex(r"[ \n\r\t]+");
    let comments = Parser::regex(r"[/][/].*").or(Parser::regex(r"[/][*].*[*][/]"));
    let ignored = whitespace.or(comments).repeat_range(0..usize::MAX);
    // Token parser constructor.
    let i = ignored.clone();
    let token = move |pattern: &str| i.clone().ignore_before(Parser::regex(pattern));
    // Token helper parsers.
    let FUNCTION = Parser::regex(r"function\b").or(ignored.clone());
    let IF = token(r"if\b");
    let ELSE = token(r"else\b");
    let RETURN = token(r"return\b");
    let VAR = token(r"var\b");
    let WHILE = token(r"while\b");
    let COMMA = token(r"[,]");
    let SEMICOLON = token(r"[;]");
    let LEFT_PAREN = token(r"[(]");
    let RIGHT_PAREN = token(r"[)]");
    let LEFT_BRACE = token(r"[{]");
    let RIGHT_BRACE = token(r"[}]");
    let NUMBER = token(r"[0-9]+").map(|matched| {
        Ok(to_string(&AstNode::integer(
            matched.parse::<i64>().unwrap(),
        ))?)
    });
    let IDENTIFIER = token(r"[a-zA-Z_][a-zA-Z0-9_]*")
        .map(|matched| Ok(to_string(&AstNode::identifier(matched))?));
    let NOT = token(r"!").map(|matched| Ok(to_string(&AstNode::not(AstNode::null()))?));
    let ASSIGN =
        token(r"=").map(|matched| Ok(to_string(&AstNode::assign("".into(), AstNode::null()))?));
    let EQUAL = token(r"==").map(|matched| {
        Ok(to_string(&AstNode::equal(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let NOT_EQUAL = token(r"!=").map(|matched| {
        Ok(to_string(&AstNode::not_equal(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let PLUS = token(r"[+]")
        .map(|matched| Ok(to_string(&AstNode::add(AstNode::null(), AstNode::null()))?));
    let MINUS = token(r"[-]").map(|matched| {
        Ok(to_string(&AstNode::subtract(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let STAR = token(r"[*]").map(|matched| {
        Ok(to_string(&AstNode::multiply(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let SLASH = token(r"[/]").map(|matched| {
        Ok(to_string(&AstNode::divide(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    // Expression parser.
    let expression = Parser::custom(parse_expression);
    // Call parser.
    let args = expression
        .clone()
        .and(
            COMMA
                .ignore_before(expression.clone())
                .repeat_range(0..usize::MAX),
        )
        .map(|matched| {
            let mut args = vec![];
            let data = from_str::<Vec<String>>(&matched)?;
            args.push(data[0].clone());
            let others = from_str::<Vec<String>>(&data[1])?;
            for o in others {
                args.push(o.clone());
            }
            Ok(to_string(&args)?)
        });
    let call = IDENTIFIER
        .clone()
        .ignore_after(LEFT_PAREN.clone())
        .and(args.clone())
        .ignore_after(RIGHT_PAREN.clone())
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let callee = data[0].clone();
            let args = from_str::<Vec<String>>(&data[1])?;
            let mut ast_args = vec![];
            for arg in &args {
                ast_args.push(from_str::<AstNode>(arg)?);
            }
            Ok(to_string(
                &AstNode::function_call(callee, ast_args),
            )?)
        });
    // Atom parser.
    let atom = call
        .clone()
        .or(IDENTIFIER.clone())
        .or(NUMBER.clone())
        .or(LEFT_PAREN
            .clone()
            .ignore_before(expression.clone())
            .ignore_after(RIGHT_PAREN.clone()));
    // Unary operator parsers.
    let unary = NOT.clone().optional().and(atom.clone()).map(|matched| {
        let data = from_str::<Vec<String>>(&matched)?;
        let atom_data = from_str::<AstNode>(&data[1])?;
        Ok(to_string(&match &data[0][..] {
            "!" => AstNode::not(atom_data),
            _ => atom_data,
        })?)
    });
    // Infix operator parsers.
    let infix = |operator_parser: Parser, term_parser: Parser| {
        term_parser
            .clone()
            .and(
                operator_parser
                    .and(term_parser.clone())
                    .repeat_range(0..usize::MAX),
            )
            .map(|matched| {
                let data = from_str::<Vec<String>>(&matched)?;
                let others = from_str::<Vec<String>>(&data[1])?;
                let mut current = from_str::<AstNode>(&data[0])?;
                for i in 0..others.len() {
                    let o = from_str::<Vec<String>>(&others[i])?; // Parse the [operator, unary]
                    let mut op = from_str::<AstNode>(&o[0])?; // Pull the operator out.
                    let t = from_str::<AstNode>(&o[1])?; // Pull the term out.
                    op.subnodes[0] = current; // Put current on the left side.
                    op.subnodes[1] = t; // Put the term on the right side.
                    current = op; // Replace current with the operator.
                }
                Ok(to_string(&current)?)
            })
    };
    let product = infix(STAR.clone().or(SLASH.clone()), unary.clone());
    let sum = infix(PLUS.clone().or(MINUS.clone()), product.clone());
    let comparison = infix(EQUAL.clone().or(NOT_EQUAL.clone()), sum.clone());
    comparison.parse(src)
}
