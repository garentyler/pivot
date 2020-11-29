use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct AstNode {
    pub value: Option<String>,
    pub kind: String,
    pub subtokens: Option<Vec<AstNode>>,
}
impl std::fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind[..] {
            "number" => write!(f, "{}", self.value.as_ref().expect("number had blank value")),
            "identifier" => write!(f, "{}", self.value.as_ref().expect("number had blank value")),
            "not" => write!(f, "!({})", self.subtokens.as_ref().expect("not had blank subtokens")[0]),
            "equal" => write!(f, "({} == ({}))", self.subtokens.as_ref().expect("equal had blank subtokens")[0], self.subtokens.as_ref().expect("equal had blank subtokens")[1]),
            "not_equal" => write!(f, "({} != {})", self.subtokens.as_ref().expect("not_equal had blank subtokens")[0], self.subtokens.as_ref().expect("not_equal had blank subtokens")[1]),
            "add" => write!(f, "({} + {})", self.subtokens.as_ref().expect("add had blank subtokens")[0], self.subtokens.as_ref().expect("add had blank subtokens")[1]),
            "subtract" => write!(f, "({} - {})", self.subtokens.as_ref().expect("subtract had blank subtokens")[0], self.subtokens.as_ref().expect("subtract had blank subtokens")[1]),
            "multiply" => write!(f, "({} * {})", self.subtokens.as_ref().expect("multiply had blank subtokens")[0], self.subtokens.as_ref().expect("multiply had blank subtokens")[1]),
            "divide" => write!(f, "({} / {})", self.subtokens.as_ref().expect("divide had blank subtokens")[0], self.subtokens.as_ref().expect("divide had blank subtokens")[1]),
            "call" => {
                write!(f, "({}(", self.value.as_ref().expect("call had blank value"))?;
                let args = self.subtokens.as_ref().expect("call had blank subtokens");
                if args.len() > 0 {
                    write!(f, "{}", args[0])?;
                    if args.len() > 1 {
                        for i in 1..args.len() {
                            write!(f, ", {}", args[i])?;
                        }
                    }
                }
                write!(f, "))")
            },
            "return" => write!(f, "return {}", self.subtokens.as_ref().expect("return had blank subtokens")[0]),
            "block" => {
                write!(f, "{{\n")?;
                let stmts = self.subtokens.as_ref().expect("block had blank subtokens");
                if stmts.len() > 0 {
                    write!(f, "{};", stmts[0])?;
                    if stmts.len() > 1 {
                        for i in 1..stmts.len() {
                            write!(f, "\n{};", stmts[i])?;
                        }
                    }
                }
                write!(f, "\n}}")
            }
            "if" => {
                let parts = self.subtokens.as_ref().expect("if had blank subtokens");
                write!(f, "(if ({}) {{{}}} else {{{}}})", parts[0], parts[1], parts[2])
            }
            "function" => {
                let parts = self.subtokens.as_ref().expect("function had blank subtokens");
                write!(f, "function {}(", self.value.as_ref().expect("function had blank value"))?;
                let params = &parts[1..];
                if params.len() > 0 {
                    write!(f, "{}", params[0])?;
                    if params.len() > 1 {
                        for i in 1..params.len() {
                            write!(f, ", {}", params[i])?;
                        }
                    }
                }
                write!(f, ") {}", parts[0])
            }
            "variable" => write!(f, "var {} = {}", self.value.as_ref().expect("var had blank value"), self.subtokens.as_ref().expect("var had blank subtokens")[0]),
            "assign" => write!(f, "{} = {}", self.value.as_ref().expect("assign had blank value"), self.subtokens.as_ref().expect("assign had blank subtokens")[0]),
            "while" => {
                let parts = self.subtokens.as_ref().expect("while had blank subtokens");
                write!(f, "while ({})\n{}\n", parts[0], parts[1])
            }
            "program" => {
                write!(f, "{{\n")?;
                let stmts = self.subtokens.as_ref().expect("program had blank subtokens");
                if stmts.len() > 0 {
                    write!(f, "{};", stmts[0])?;
                    if stmts.len() > 1 {
                        for i in 1..stmts.len() {
                            write!(f, "\n{};", stmts[i])?;
                        }
                    }
                }
                write!(f, "\n}}")
            }
            _ => write!(f, "(unknown node type {})", self.kind),
        }
    }
}
impl AstNode {
    pub fn emit(&self, f: &mut dyn Write) -> Result<(), std::fmt::Error> {
        match &self.kind[..] {
            "number" => write!(f, "i32.const {}\n", self.value.as_ref().expect("number had blank value")),
            "add" => {
                let subtokens = self.subtokens.as_ref().expect("add had blank subtokens");
                subtokens[0].emit(f)?;
                subtokens[1].emit(f)?;
                write!(f, "i32.add\n")
            },
            _ => Ok(())
        }
    }

    pub fn number(num: i32) -> AstNode {
        AstNode {
            value: Some(num.to_string()),
            kind: "number".into(),
            subtokens: None,
        }
    }
    pub fn identifier<T: Into<String>>(id: T) -> AstNode {
        AstNode {
            value: Some(id.into()),
            kind: "identifier".into(),
            subtokens: None,
        }
    }
    pub fn not(operand: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "not".into(),
            subtokens: Some(vec![operand]),
        }
    }
    pub fn equal(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "equal".into(),
            subtokens: Some(vec![left, right]),
        }
    }
    pub fn not_equal(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "not_equal".into(),
            subtokens: Some(vec![left, right]),
        }
    }
    pub fn add(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "add".into(),
            subtokens: Some(vec![left, right]),
        }
    }
    pub fn subtract(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "subtract".into(),
            subtokens: Some(vec![left, right]),
        }
    }
    pub fn multiply(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "multiply".into(),
            subtokens: Some(vec![left, right]),
        }
    }
    pub fn divide(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "divide".into(),
            subtokens: Some(vec![left, right]),
        }
    }
    pub fn call<T: Into<String>>(callee: T, args: Vec<AstNode>) -> AstNode {
        AstNode {
            value: Some(callee.into()),
            kind: "call".into(),
            subtokens: Some(args),
        }
    }
    pub fn r#return(operand: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "return".into(),
            subtokens: Some(vec![operand]),
        }
    }
    pub fn block(statements: Vec<AstNode>) -> AstNode {
        AstNode {
            value: None,
            kind: "block".into(),
            subtokens: Some(statements),
        }
    }
    pub fn r#if(conditional: AstNode, consequence: AstNode, alternative: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "if".into(),
            subtokens: Some(vec![conditional, consequence, alternative]),
        }
    }
    pub fn function<T: Into<String>>(name: T, parameters: Vec<(T, T)>, body: AstNode) -> AstNode {
        // Turn the parameter strings into ids.
        let mut params = vec![];
        params.push(body); // First one will always be the body.
        for p in parameters {
            params.push(AstNode::identifier(p));
        }
        AstNode {
            value: Some(name.into()),
            kind: "function".into(),
            subtokens: Some(params),
        }
    }
    pub fn variable<T: Into<String>>(name: T, value: AstNode) -> AstNode {
        AstNode {
            value: Some(name.into()),
            kind: "variable".into(),
            subtokens: Some(vec![value]),
        }
    }
    pub fn assign<T: Into<String>>(name: T, value: AstNode) -> AstNode {
        AstNode {
            value: Some(name.into()),
            kind: "assign".into(),
            subtokens: Some(vec![value]),
        }
    }
    pub fn r#while(conditional: AstNode, body: AstNode) -> AstNode {
        AstNode {
            value: None,
            kind: "while".into(),
            subtokens: Some(vec![conditional, body]),
        }
    }
    pub fn program(statements: Vec<AstNode>) -> AstNode {
        AstNode {
            value: None,
            kind: "program".into(),
            subtokens: Some(statements),
        }
    }
}
