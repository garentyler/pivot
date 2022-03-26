use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    // Primitives
    Integer(i32),
    Identifier(String),
    String(String),
    Boolean(bool),
    // Unary operators
    Not {
        operand: Box<AstNode>,
    },
    // Infix operators
    Equal {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Add {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Subtract {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Multiply {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Divide {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Assign {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    // Control flow
    Block {
        statements: Vec<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        consequence: Box<AstNode>,
        alternative: Option<Box<AstNode>>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    // Functions and variables
    FunctionCall {
        identifier: Box<AstNode>,
        arguments: Vec<AstNode>,
    },
    FunctionReturn {
        value: Box<AstNode>,
    },
    FunctionDefinition {
        identifier: Box<AstNode>,
        arguments: Vec<AstNode>,
        body: Box<AstNode>,
    },
    VariableDeclaration {
        identifier: Box<AstNode>,
    },
    // Other
    Import {
        identifier: Box<AstNode>,
    },
    Null,
}
impl AstNode {
    // Primitives
    pub fn integer(value: i32) -> AstNode {
        AstNode::Integer(value)
    }
    pub fn identifier(value: String) -> AstNode {
        AstNode::Identifier(value)
    }
    pub fn string(value: String) -> AstNode {
        AstNode::String(value)
    }
    pub fn boolean(value: bool) -> AstNode {
        AstNode::Boolean(value)
    }
    // Unary operators
    pub fn not(operand: AstNode) -> AstNode {
        AstNode::Not {
            operand: Box::new(operand),
        }
    }
    // Infix operators
    pub fn equal(left: AstNode, right: AstNode) -> AstNode {
        AstNode::Equal {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn not_equal(left: AstNode, right: AstNode) -> AstNode {
        AstNode::not(AstNode::Equal {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
    pub fn add(left: AstNode, right: AstNode) -> AstNode {
        AstNode::Add {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn subtract(left: AstNode, right: AstNode) -> AstNode {
        AstNode::Subtract {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn multiply(left: AstNode, right: AstNode) -> AstNode {
        AstNode::Multiply {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn divide(left: AstNode, right: AstNode) -> AstNode {
        AstNode::Divide {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn assign(left: AstNode, right: AstNode) -> AstNode {
        AstNode::Assign {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    // Control flow
    pub fn block(statements: Vec<AstNode>) -> AstNode {
        AstNode::Block { statements }
    }
    pub fn if_statement(
        condition: AstNode,
        consequence: AstNode,
        alternative: Option<AstNode>,
    ) -> AstNode {
        AstNode::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: alternative.and_then(|alt| Some(Box::new(alt))),
        }
    }
    pub fn while_loop(condition: AstNode, body: AstNode) -> AstNode {
        AstNode::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
    // Functions and variables
    pub fn function_call(identifier: AstNode, arguments: Vec<AstNode>) -> AstNode {
        AstNode::FunctionCall {
            identifier: Box::new(identifier),
            arguments,
        }
    }
    pub fn function_return(value: AstNode) -> AstNode {
        AstNode::FunctionReturn {
            value: Box::new(value),
        }
    }
    pub fn function_definition(
        identifier: AstNode,
        arguments: Vec<AstNode>,
        body: AstNode,
    ) -> AstNode {
        AstNode::FunctionDefinition {
            identifier: Box::new(identifier),
            arguments,
            body: Box::new(body),
        }
    }
    pub fn variable_declaration(identifier: AstNode) -> AstNode {
        AstNode::VariableDeclaration {
            identifier: Box::new(identifier),
        }
    }
    // Other
    pub fn import(identifier: AstNode) -> AstNode {
        AstNode::Import {
            identifier: Box::new(identifier),
        }
    }
    pub fn null() -> AstNode {
        AstNode::Null
    }
}