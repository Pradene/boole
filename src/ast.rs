use std::collections::LinkedList;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Operator {
    Or,          // Logical OR (∨)
    And,         // Logical AND (∧)
    Not,         // Logical NOT (¬)
    Xor,         // Logical XOR (exclusive OR ⊕)
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

#[derive(Debug, Clone)]
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
    
    pub fn to_nnf(&self) -> AstNode {
        match self {
            // Variables remain unchanged
            AstNode::Variable(_) => self.clone(),
            
            // Handle unary operators (NOT)
            AstNode::UnaryOperator(Operator::Not, child) => {
                match &**child {
                    // Double negation elimination: ¬¬A → A
                    AstNode::UnaryOperator(Operator::Not, grandchild) => {
                        grandchild.to_nnf()
                    },
                    
                    // De Morgan's laws: ¬(A ∧ B) → (¬A ∨ ¬B)
                    AstNode::BinaryOperator(Operator::And, left, right) => {
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf()),
                            Box::new(AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf())
                        )
                    },
                    
                    // De Morgan's laws: ¬(A ∨ B) → (¬A ∧ ¬B)
                    AstNode::BinaryOperator(Operator::Or, left, right) => {
                        AstNode::BinaryOperator(
                            Operator::And,
                            Box::new(AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf()),
                            Box::new(AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf())
                        )
                    },
                    
                    // Handle implication: ¬(A → B) ≡ A ∧ ¬B
                    AstNode::BinaryOperator(Operator::Implies, left, right) => {
                        AstNode::BinaryOperator(
                            Operator::And,
                            Box::new(left.to_nnf()),
                            Box::new(AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf())
                        )
                    },
                    
                    // Handle equivalence: ¬(A ↔ B) ≡ (A ∧ ¬B) ∨ (¬A ∧ B)
                    AstNode::BinaryOperator(Operator::Iff, left, right) => {
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(left.to_nnf()),
                                Box::new(AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf())
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf()),
                                Box::new(right.to_nnf())
                            ))
                        )
                    },
                    
                    // Handle XOR: ¬(A ⊕ B) ≡ (A ↔ B) ≡ (A ∧ B) ∨ (¬A ∧ ¬B)
                    AstNode::BinaryOperator(Operator::Xor, left, right) => {
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(left.to_nnf()),
                                Box::new(right.to_nnf())
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf()),
                                Box::new(AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf())
                            ))
                        )
                    },
                    
                    // For variables, keep the NOT
                    AstNode::Variable(_) => self.clone(),

                    _ => panic!("Error"),
                }
            },
            
            // Handle binary operators
            AstNode::BinaryOperator(op, left, right) => {
                match op {
                    // AND and OR just need their children converted
                    Operator::And | Operator::Or => {
                        AstNode::BinaryOperator(
                            op.clone(),
                            Box::new(left.to_nnf()),
                            Box::new(right.to_nnf())
                        )
                    },
                    
                    // A → B ≡ ¬A ∨ B
                    Operator::Implies => {
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf()),
                            Box::new(right.to_nnf())
                        )
                    },
                    
                    // A ↔ B ≡ (A ∧ B) ∨ (¬A ∧ ¬B)
                    Operator::Iff => {
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(left.to_nnf()),
                                Box::new(right.to_nnf())
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf()),
                                Box::new(AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf())
                            ))
                        )
                    },
                    
                    // A ⊕ B ≡ (A ∧ ¬B) ∨ (¬A ∧ B)
                    Operator::Xor => {
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(left.to_nnf()),
                                Box::new(AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf())
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf()),
                                Box::new(right.to_nnf())
                            ))
                        )
                    },
                    
                    // NOT should never appear as a binary operator
                    _ => panic!("Error"),
                }
            },
            _ => panic!("Error"),
        }
    }

    pub fn to_rpn(&self) -> String {
        match self {
            AstNode::Variable(var) => var.clone(),
            
            AstNode::UnaryOperator(op, child) => {
                format!("{}{}", 
                    child.to_rpn(),
                    match op {
                        Operator::Not => "!",
                        _ => panic!("Unexpected unary operator"),
                    }
                )
            },
            
            AstNode::BinaryOperator(op, left, right) => {
                format!("{}{}{}", 
                    left.to_rpn(),
                    right.to_rpn(),
                    match op {
                        Operator::Or => "|",
                        Operator::And => "&",
                        Operator::Xor => "^",
                        Operator::Iff => "=",
                        Operator::Implies => ">",
                        _ => panic!("Unexpected binary operator"),
                    }
                )
            },
        }
    }
}