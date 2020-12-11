use ron::from_str;
use crate::ast::{AstNode, AstNodeKind};

pub struct SymbolGenerator {
    counter: usize,
}
impl SymbolGenerator {
    pub fn new() -> SymbolGenerator {
        SymbolGenerator {
            counter: 0,
        }
    }
    pub fn next(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}

pub trait Wasm {
    fn emit(&self, symbol_generator: &mut SymbolGenerator) -> String;
}
impl Wasm for AstNode {
    fn emit(&self, s: &mut SymbolGenerator) -> String {
        use AstNodeKind::*;
        match self.kind {
            // Primitives
            Integer => format!("(i32.const {})", self.value),
            Identifier => format!("(get_local ${})", self.value),
            // Unary operators
            Not => format!("(i32.eq (i32.const 0) {})", self.subnodes[1].emit(s)),
            // Infix operators
            NotEqual => format!("(i32.ne {} {})", self.subnodes[0].emit(s), self.subnodes[1].emit(s)),
            Equal => format!("(i32.eq {} {})", self.subnodes[0].emit(s), self.subnodes[1].emit(s)),
            Add => format!("(i32.add {} {})", self.subnodes[0].emit(s), self.subnodes[1].emit(s)),
            Subtract => format!("(i32.sub {} {})", self.subnodes[0].emit(s), self.subnodes[1].emit(s)),
            Multiply => format!("(i32.mul {} {})", self.subnodes[0].emit(s), self.subnodes[1].emit(s)),
            Divide => format!("(i32.div_s {} {})", self.subnodes[0].emit(s), self.subnodes[1].emit(s)),
            // Control flow
            Block => {
                let mut out = String::new();
                for node in &self.subnodes {
                    out += "";
                    out += &node.emit(s);
                }
                out
            }
            IfStatement => {
                let mut out = String::new();
                out += &format!("(if {} (then {})", self.subnodes[0].emit(s), self.subnodes[1].emit(s)); // Emit the conditional and consequence.
                if let Some(alternative) = self.subnodes.get(2) {
                    out += &format!(" (else {})", alternative.emit(s)); // Emit the alternative.
                }
                out += ")";
                out
            }
            WhileLoop => {
                let loop_symbol = format!("while{}", s.next()); // TODO: Make generate unique symbol for nested loops.
                let mut out = String::new();
                out += &format!("(block ${}_wrapper", loop_symbol);
                out += &format!(" (loop ${}_loop", loop_symbol);
                out += &format!(" {}", self.subnodes[1].emit(s));
                out += &format!(" (br_if ${}_wrapper (i32.eq (i32.const 0) {}))", loop_symbol, self.subnodes[0].emit(s));
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
                    out += &node.emit(s);
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
                    out += &n.emit(s);
                }
                out += ")";
                out
            },
            FunctionReturn => format!("{} (return)", self.subnodes[0].emit(s)),
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
                    out += &n.emit(s);
                }
                out += ")";
                out
            }
            VariableDeclaration => format!("(local ${} i32)", self.value),
            Assign => format!("(set_local ${} {})", self.value, self.subnodes[0].emit(s)),
            // Blank node / other
            Null | _ => "".into(),
        }
    }
}
