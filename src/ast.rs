use serde::{Deserialize, Serialize};
use ron::from_str;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AstNodeKind {
    // Primitives
    Integer,
    Identifier,
    // Unary operators
    Not,
    // Infix operators
    NotEqual,
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    // Control flow
    Block,
    IfStatement,
    WhileLoop,
    Program,
    // Functions and variables
    FunctionCall,
    FunctionReturn,
    FunctionDefinition,
    VariableDefinition,
    VariableDeclaration,
    Assign,
    // Blank node
    Null,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub value: String,
    pub subnodes: Vec<AstNode>,
}
impl AstNode {
    pub fn new(kind: AstNodeKind, value: String, subnodes: Vec<AstNode>) -> AstNode {
        AstNode {
            kind,
            value,
            subnodes,
        }
    }
    pub fn emit(&self) -> String {
        use AstNodeKind::*;
        match self.kind {
            // Primitives
            Integer => format!("(i32.const {})", self.value),
            Identifier => format!("(get_local ${})", self.value),
            // Unary operators
            Not => format!("(i32.eq (i32.const 0) {})", self.subnodes[1].emit()),
            // Infix operators
            NotEqual => format!("(i32.ne {} {})", self.subnodes[0].emit(), self.subnodes[1].emit()),
            Equal => format!("(i32.eq {} {})", self.subnodes[0].emit(), self.subnodes[1].emit()),
            Add => format!("(i32.add {} {})", self.subnodes[0].emit(), self.subnodes[1].emit()),
            Subtract => format!("(i32.sub {} {})", self.subnodes[0].emit(), self.subnodes[1].emit()),
            Multiply => format!("(i32.mul {} {})", self.subnodes[0].emit(), self.subnodes[1].emit()),
            Divide => format!("(i32.div_s {} {})", self.subnodes[0].emit(), self.subnodes[1].emit()),
            // Control flow
            Block => {
                let mut out = String::new();
                for node in &self.subnodes {
                    out += "";
                    out += &node.emit();
                }
                out
            }
            IfStatement => {
                let mut out = String::new();
                out += &format!("(if {} (then {})", self.subnodes[0].emit(), self.subnodes[1].emit()); // Emit the conditional and consequence.
                if let Some(alternative) = self.subnodes.get(2) {
                    out += &format!(" (else {})", alternative.emit()); // Emit the alternative.
                }
                out += ")";
                out
            }
            WhileLoop => {
                let loop_symbol = "while_loop"; // TODO: Make generate unique symbol for nested loops.
                let mut out = String::new();
                out += &format!("(block ${}_wrapper", loop_symbol);
                out += &format!(" (loop ${}_loop", loop_symbol);
                out += &format!(" {}", self.subnodes[1].emit());
                out += &format!(" (br_if ${}_wrapper (i32.eq (i32.const 0) {}))", loop_symbol, self.subnodes[0].emit());
                out += &format!(" (br ${}_loop)", loop_symbol);
                out += "))";
                out
            }
            Program => {
                let mut out = String::new();
                out += "(module";
                let mut exported = vec![];
                for node in &self.subnodes {
                    out += " ";
                    out += &node.emit();
                    if node.kind == FunctionDefinition {
                        exported.push(node.value.clone());
                    }
                }
                for export in exported {
                    out += &format!(" (export \"{0}\" (func ${0}))", export);
                }
                out += ")";
                out
            }
            // Functions and variables
            FunctionCall => {
                let mut out = String::new();
                out += &format!("(call ${}", from_str::<AstNode>(&self.value).unwrap().value);
                for n in &self.subnodes {
                    out += " ";
                    out += &n.emit();
                }
                out += ")";
                out
            },
            FunctionReturn => format!("{} (return)", self.subnodes[0].emit()),
            FunctionDefinition => {
                let mut out = String::new();
                out += &format!("(func ${}", self.value);
                let body = self.subnodes[0].clone();
                for n in &self.subnodes[1..] {
                    out += &format!(" (param ${} i32)", n.value);
                }
                let mut func_returns_value = false;
                let mut index = 0;
                loop {
                    if index >= body.subnodes.len() {
                        break;
                    }
                    match body.subnodes[index].kind {
                        AstNodeKind::FunctionReturn => func_returns_value = true,
                        _ => {}
                    }
                    index += 1;
                }
                if func_returns_value {
                    out += " (result i32)";
                }
                for n in &body.subnodes {
                    out += " ";
                    out += &n.emit();
                }
                out += ")";
                out
            }
            VariableDeclaration => format!("(local ${} i32)", self.value),
            Assign => format!("(set_local ${} {})", self.value, self.subnodes[0].emit()),
            // Blank node / other
            Null | _ => "".into(),
        }
    }

    // Primitives
    pub fn integer(num: i64) -> AstNode {
        AstNode {
            kind: AstNodeKind::Integer,
            value: num.to_string(),
            subnodes: vec![],
        }
    }
    pub fn identifier(id: String) -> AstNode {
        AstNode {
            kind: AstNodeKind::Identifier,
            value: id,
            subnodes: vec![],
        }
    }
    // Unary operators
    pub fn not(operand: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::Not,
            value: "not".into(),
            subnodes: vec![operand],
        }
    }
    // Infix operators
    pub fn not_equal(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::NotEqual,
            value: "not_equal".into(),
            subnodes: vec![left, right],
        }
    }
    pub fn equal(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::Equal,
            value: "equal".into(),
            subnodes: vec![left, right],
        }
    }
    pub fn add(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::Add,
            value: "add".into(),
            subnodes: vec![left, right],
        }
    }
    pub fn subtract(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::Subtract,
            value: "subtract".into(),
            subnodes: vec![left, right],
        }
    }
    pub fn multiply(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::Multiply,
            value: "multiply".into(),
            subnodes: vec![left, right],
        }
    }
    pub fn divide(left: AstNode, right: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::Divide,
            value: "divide".into(),
            subnodes: vec![left, right],
        }
    }
    // Control flow
    pub fn block(statements: Vec<AstNode>) -> AstNode {
        AstNode {
            kind: AstNodeKind::Block,
            value: "block".into(),
            subnodes: statements,
        }
    }
    pub fn if_statement(
        conditional: AstNode,
        consequence: AstNode,
        alternative: AstNode,
    ) -> AstNode {
        AstNode {
            kind: AstNodeKind::IfStatement,
            value: "if_statement".into(),
            subnodes: vec![conditional, consequence, alternative],
        }
    }
    pub fn while_loop(conditional: AstNode, body: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::WhileLoop,
            value: "while_loop".into(),
            subnodes: vec![conditional, body],
        }
    }
    pub fn program(statements: Vec<AstNode>) -> AstNode {
        AstNode {
            kind: AstNodeKind::Program,
            value: "program".into(),
            subnodes: statements,
        }
    }
    // Functions and variables
    pub fn function_call(name: String, parameters: Vec<AstNode>) -> AstNode {
        AstNode {
            kind: AstNodeKind::FunctionCall,
            value: name,
            subnodes: parameters,
        }
    }
    pub fn function_return(operand: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::FunctionReturn,
            value: "return".into(),
            subnodes: vec![operand],
        }
    }
    pub fn function_definition(name: String, parameters: Vec<AstNode>, body: AstNode) -> AstNode {
        let mut params = vec![body];
        for p in parameters {
            params.push(p);
        }
        AstNode {
            kind: AstNodeKind::FunctionDefinition,
            value: name,
            subnodes: params,
        }
    }
    pub fn variable_definition(name: String, value: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::VariableDefinition,
            value: name,
            subnodes: vec![value],
        }
    }
    pub fn variable_declaration(name: String) -> AstNode {
        AstNode {
            kind: AstNodeKind::VariableDeclaration,
            value: name,
            subnodes: vec![],
        }
    }
    pub fn assign(name: String, value: AstNode) -> AstNode {
        AstNode {
            kind: AstNodeKind::Assign,
            value: name,
            subnodes: vec![value],
        }
    }
    // Blank node
    pub fn null() -> AstNode {
        AstNode {
            kind: AstNodeKind::Null,
            value: "".into(),
            subnodes: vec![],
        }
    }

    // Other
    pub fn pretty_print(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        for _ in 0..indent {
            write!(f, " ")?;
        }
        write!(f, "{{\n")?;
        for _ in 0..indent + 2 {
            write!(f, " ")?;
        }
        write!(f, "kind: {:?}\n", self.kind)?;
        for _ in 0..indent + 2 {
            write!(f, " ")?;
        }
        write!(f, "value: {:?}\n", self.value)?;
        if self.subnodes.len() > 0 {
            for _ in 0..indent + 2 {
                write!(f, " ")?;
            }
            write!(f, "subnodes: [\n")?;
            for subnode in &self.subnodes {
                subnode.pretty_print(f, indent + 4)?;
                write!(f, ",\n")?;
            }
            for _ in 0..indent + 2 {
                write!(f, " ")?;
            }
            write!(f, "]\n")?;
        }
        for _ in 0..indent {
            write!(f, " ")?;
        }
        write!(f, "}}")
    }
}
impl std::fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}
