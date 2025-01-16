use std::collections::LinkedList;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Operator {
    Or,          // Logical OR (∨)
    And,         // Logical AND (∧)
    Not,         // Logical NOT (¬)
    Xor,         // Logical XOR (exclusive OR)
    Iff,         // Logical Equivalence (↔)
    Implies,     // Logical Implication (→)
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            Operator::Or => "|",
            Operator::And => "&",
            Operator::Not => "!",
            Operator::Xor => "^",
            Operator::Iff => "=",
            Operator::Implies => ">",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug)]
pub enum AstNode {
    Variable(String),
    BinaryOperator(Operator, Box<AstNode>, Box<AstNode>),
    UnaryOperator(Operator, Box<AstNode>),
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Variable(name) => write!(f, "{}", name),
            AstNode::BinaryOperator(op, left, right) => {
                write!(f, "({} {} {})", left, op, right)  // Format as "(left operator right)"
            }
            AstNode::UnaryOperator(op, operand) => {
                write!(f, "{}{}", op, operand)  // Format as "operator operand"
            }
        }
    }
}

impl TryFrom<&str> for AstNode {
    type Error = String;

    fn try_from(rpn: &str) -> Result<Self, Self::Error> {
        let mut stack: LinkedList<AstNode> = LinkedList::new();
        
        for token in rpn.chars() {
            match token {
                '1' => stack.push_back(AstNode::Variable(String::from("1"))),  // '1' is true
                '0' => stack.push_back(AstNode::Variable(String::from("0"))),  // '0' is false
                '|' => {
                    let right = stack.pop_back().ok_or_else(|| "Missing operand for OR".to_string())?;
                    let left = stack.pop_back().ok_or_else(|| "Missing operand for OR".to_string())?;
                    stack.push_back(AstNode::BinaryOperator(Operator::Or, Box::new(left), Box::new(right)));
                }
                '&' => {
                    let right = stack.pop_back().ok_or_else(|| "Missing operand for AND".to_string())?;
                    let left = stack.pop_back().ok_or_else(|| "Missing operand for AND".to_string())?;
                    stack.push_back(AstNode::BinaryOperator(Operator::And, Box::new(left), Box::new(right)));
                }
                '!' => {
                    let element = stack.pop_back().ok_or_else(|| "Missing operand for NOT".to_string())?;
                    stack.push_back(AstNode::UnaryOperator(Operator::Not, Box::new(element)));
                }
                '^' => {
                    let right = stack.pop_back().ok_or_else(|| "Missing operand for XOR".to_string())?;
                    let left = stack.pop_back().ok_or_else(|| "Missing operand for XOR".to_string())?;
                    stack.push_back(AstNode::BinaryOperator(Operator::Xor, Box::new(left), Box::new(right)));
                }
                '=' => {
                    let right = stack.pop_back().ok_or_else(|| "Missing operand for IFF".to_string())?;
                    let left = stack.pop_back().ok_or_else(|| "Missing operand for IFF".to_string())?;
                    stack.push_back(AstNode::BinaryOperator(Operator::Iff, Box::new(left), Box::new(right)));
                }
                '>' => {
                    let right = stack.pop_back().ok_or_else(|| "Missing operand for IMPLIES".to_string())?;
                    let left = stack.pop_back().ok_or_else(|| "Missing operand for IMPLIES".to_string())?;
                    stack.push_back(AstNode::BinaryOperator(Operator::Implies, Box::new(left), Box::new(right)));
                }
                _ => return Err(format!("Unknown token: {}", token)),
            }
        }

        if stack.len() != 1 {
            return Err(format!("Error: The stack should contain exactly one element, but it contains {}", stack.len()));
        }
        
        Ok(stack.pop_back().unwrap())
    }
}

impl AstNode {
    pub fn to_string(&self) -> String {
        match self {
            AstNode::Variable(name) => name.clone(), // For variables, just return the name
            AstNode::BinaryOperator(op, left, right) => {
                let left_str = left.to_string();   // Recursively get the left operand's string
                let right_str = right.to_string(); // Recursively get the right operand's string
                let op_str = match op {
                    Operator::And => "&",
                    Operator::Or => "|",
                    Operator::Xor => "^",
                    Operator::Iff => "=",
                    Operator::Implies => ">",
                    _ => panic!("Unsupported operator"),
                };
                format!("({} {} {})", left_str, op_str, right_str) // Format as "(left operator right)"
            }
            AstNode::UnaryOperator(op, operand) => {
                let operand_str = operand.to_string(); // Recursively get the operand's string
                let op_str = match op {
                    Operator::Not => "!", // Represent NOT with "!"
                    _ => panic!("Unsupported unary operator"),
                };
                format!("{}{}", op_str, operand_str) // Format as "!operand"
            }
        }
    }
}