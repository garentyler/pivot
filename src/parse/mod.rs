mod combinators;

use crate::ast::{AstNode, AstNodeKind};
use combinators::Parser;
use ron::{from_str, to_string};

pub fn parse<T: Into<String>>(src: T) -> AstNode {
    let src: String = src.into();
    let whitespace = Parser::regex(r"[ \n\r\t]+");
    let comments = Parser::regex(r"[/][/].*").or(Parser::regex(r"[/][*].*[*][/]"));
    let ignored = whitespace.or(comments).repeat_range(0..usize::MAX);
    let statement = ignored
        .optional()
        .ignore()
        .and(Parser::custom(parse_statement));
    let parse_program = statement
        .clone()
        .repeat_range(0..usize::MAX)
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let mut statements = vec![];
            for d in data {
                statements.push(from_str::<AstNode>(&d)?);
            }
            Ok(to_string(&AstNode::program(statements))?)
        });
    from_str::<AstNode>(&parse_program.parse(src).unwrap().0).unwrap()
}
fn parse_statement(src: String) -> Result<(String, String), String> {
    let whitespace = Parser::regex(r"[ \n\r\t]+");
    let comments = Parser::regex(r"[/][/].*").or(Parser::regex(r"[/][*].*[*][/]"));
    let ignored = whitespace.or(comments).repeat_range(0..usize::MAX);
    // Token parser constructor.
    let i = ignored.clone();
    let token = move |pattern: &str| i.clone().ignore().and(Parser::regex(pattern));
    // Token helper parsers.
    let function = Parser::regex(r"function\b").or(ignored.clone());
    let return_token = token(r"return\b");
    let semicolon = token(r"[;]");
    let if_token = token(r"if\b");
    let else_token = token(r"else\b");
    let left_paren = token(r"[(]");
    let right_paren = token(r"[)]");
    let left_brace = token(r"[{]");
    let right_brace = token(r"[}]");
    let while_token = token(r"while\b");
    let var = token(r"var\b");
    let identifier = token(r"[a-zA-Z_][a-zA-Z0-9_]*")
        .map(|matched| Ok(to_string(&AstNode::identifier(matched))?));
    let assign =
        token(r"=").map(|_matched| Ok(to_string(&AstNode::assign("".into(), AstNode::null()))?));
    let comma = token(r"[,]");
    let expression = Parser::custom(parse_expression);
    let statement = Parser::custom(parse_statement);
    let return_statement = return_token
        .clone()
        .ignore()
        .and(expression.clone())
        .and(semicolon.clone().ignore())
        .map(|matched| {
            let data = from_str::<AstNode>(&matched)?;
            Ok(to_string(&AstNode::function_return(data))?)
        });
    let expression_statement = expression.clone().and(semicolon.clone().ignore());
    let if_statement = if_token
        .clone()
        .ignore()
        .and(
            left_paren
                .clone()
                .ignore()
                .and(expression.clone())
                .and(right_paren.clone().ignore()),
        )
        .and(statement.clone())
        .and(
            else_token
                .clone()
                .ignore()
                .and(statement.clone())
                .optional(),
        )
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let alternative = from_str::<Vec<String>>(&data[1])?;
            let alternative = match alternative.get(0) {
                Some(s) => from_str::<AstNode>(&s)?,
                None => AstNode::null(),
            };
            let others = from_str::<Vec<String>>(&data[0])?;
            let conditional = from_str::<AstNode>(&others[0])?;
            let consequence = from_str::<AstNode>(&others[1])?;
            Ok(to_string(&AstNode::if_statement(
                conditional,
                consequence,
                alternative,
            ))?)
        });
    let while_statement = while_token
        .clone()
        .ignore()
        .and(
            left_paren
                .clone()
                .ignore()
                .and(expression.clone())
                .and(right_paren.clone().ignore()),
        )
        .and(statement.clone())
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let conditional = from_str::<AstNode>(&data[0])?;
            let body = from_str::<AstNode>(&data[1])?;
            Ok(to_string(&AstNode::while_loop(conditional, body))?)
        });
    let var_statement = var
        .clone()
        .ignore()
        .and(identifier.clone())
        .and(assign.clone().ignore())
        .and(expression.clone())
        .and(semicolon.clone().ignore())
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let name = from_str::<AstNode>(&data[0])?.value;
            let value = from_str::<AstNode>(&data[1])?;
            Ok(to_string(&AstNode::variable_definition(name, value))?)
        });
    let assignment_statement = identifier
        .clone()
        .and(assign.clone().ignore())
        .and(expression.clone())
        .and(semicolon.clone().ignore())
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let name = from_str::<AstNode>(&data[0])?.value;
            let value = from_str::<AstNode>(&data[1])?;
            Ok(to_string(&AstNode::assign(name, value))?)
        });
    let block_statement = left_brace
        .clone()
        .ignore()
        .and(statement.clone().repeat_range(0..usize::MAX))
        .and(right_brace.clone().ignore())
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let mut statements = vec![];
            for d in data {
                statements.push(from_str::<AstNode>(&d)?);
            }
            Ok(to_string(&AstNode::block(statements))?)
        });
    let args = identifier
        .clone()
        .and(
            comma
                .clone()
                .ignore()
                .and(identifier.clone())
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
    let function_statement = function
        .clone()
        .ignore()
        .and(identifier.clone())
        .and(
            left_paren
                .clone()
                .ignore()
                .and(args.clone().optional())
                .and(right_paren.clone().ignore()),
        )
        .and(block_statement.clone())
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let mut body = from_str::<AstNode>(&data[1])?;
            let data = from_str::<Vec<String>>(&data[0])?;
            let name = from_str::<AstNode>(&data[0])?.value;
            let params = from_str::<Vec<String>>(&data[1])?;
            let mut parameters = vec![];
            if params.len() != 0 {
                for p in from_str::<Vec<String>>(&params[0])? {
                    parameters.push(from_str::<AstNode>(&p)?);
                }
            }
            // Hoist variable definitions.
            let mut vars = vec![];
            let mut others = vec![];
            for node in &body.subnodes {
                match node.kind {
                    AstNodeKind::VariableDefinition => {
                        vars.push(AstNode::variable_declaration(node.value.clone()));
                        others.push(AstNode::assign(
                            node.value.clone(),
                            node.subnodes[0].clone(),
                        ))
                    }
                    _ => others.push(node.clone()),
                }
            }
            vars.append(&mut others);
            body.subnodes = vars;
            Ok(to_string(&AstNode::function_definition(
                name, parameters, body,
            ))?)
        });
    return_statement
        .clone()
        .or(if_statement.clone())
        .or(while_statement.clone())
        .or(var_statement.clone())
        .or(assignment_statement.clone())
        .or(block_statement.clone())
        .or(function_statement.clone())
        .or(expression_statement.clone())
        .parse(src)
}
fn parse_expression(src: String) -> Result<(String, String), String> {
    let whitespace = Parser::regex(r"[ \n\r\t]+");
    let comments = Parser::regex(r"[/][/].*").or(Parser::regex(r"[/][*].*[*][/]"));
    let ignored = whitespace.or(comments).repeat_range(0..usize::MAX);
    // Token parser constructor.
    let i = ignored.clone();
    let token = move |pattern: &str| i.clone().ignore().and(Parser::regex(pattern));
    // Token helper parsers.
    let comma = token(r"[,]");
    let left_paren = token(r"[(]");
    let right_paren = token(r"[)]");
    let number = token(r"[0-9]+").map(|matched| {
        Ok(to_string(&AstNode::integer(
            matched.parse::<i64>().unwrap(),
        ))?)
    });
    let identifier = token(r"[a-zA-Z_][a-zA-Z0-9_]*")
        .map(|matched| Ok(to_string(&AstNode::identifier(matched))?));
    let not = token(r"!").map(|_matched| Ok(to_string(&AstNode::not(AstNode::null()))?));
    let equal = token(r"==").map(|_matched| {
        Ok(to_string(&AstNode::equal(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let not_equal = token(r"!=").map(|_matched| {
        Ok(to_string(&AstNode::not_equal(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let plus = token(r"[+]")
        .map(|_matched| Ok(to_string(&AstNode::add(AstNode::null(), AstNode::null()))?));
    let minus = token(r"[-]").map(|_matched| {
        Ok(to_string(&AstNode::subtract(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let star = token(r"[*]").map(|_matched| {
        Ok(to_string(&AstNode::multiply(
            AstNode::null(),
            AstNode::null(),
        ))?)
    });
    let slash = token(r"[/]").map(|_matched| {
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
            comma
                .clone()
                .ignore()
                .and(expression.clone())
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
    let call = identifier
        .clone()
        .and(left_paren.clone().ignore())
        .and(args.clone().optional())
        .and(right_paren.clone().ignore())
        .map(|matched| {
            let data = from_str::<Vec<String>>(&matched)?;
            let callee = data[0].clone();
            let args = from_str::<Vec<String>>(&data[1])?;
            let mut ast_args = vec![];
            if let Some(args) = args.get(0) {
                let args = from_str::<Vec<String>>(&args)?;
                for arg in &args {
                    ast_args.push(from_str::<AstNode>(arg)?);
                }
            }
            Ok(to_string(&AstNode::function_call(callee, ast_args))?)
        });
    // Atom parser.
    let atom = call
        .clone()
        .or(identifier.clone())
        .or(number.clone())
        .or(left_paren
            .clone()
            .ignore()
            .and(expression.clone())
            .and(right_paren.clone().ignore()));
    // Unary operator parsers.
    let unary = not.clone().optional().and(atom.clone()).map(|matched| {
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
    let product = infix(star.clone().or(slash.clone()), unary.clone());
    let sum = infix(plus.clone().or(minus.clone()), product.clone());
    let comparison = infix(equal.clone().or(not_equal.clone()), sum.clone());
    comparison.parse(src)
}
