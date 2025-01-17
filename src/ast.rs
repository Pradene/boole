use std::collections::{LinkedList, HashMap};
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
    Variable(char),
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
        
        // Step 1: Iterate over the characters in reverse order to build the stack
        for token in rpn.chars() {
            match token {
                'A'..='Z' => stack.push_back(AstNode::Variable(token)),
                '|' | '&' | '^' | '=' | '>' => {
                    // Ensure there are at least two operands for binary operators
                    let right = stack.pop_back().ok_or_else(|| "Missing operand for operator".to_string())?;
                    let left = stack.pop_back().ok_or_else(|| "Missing operand for operator".to_string())?;
                    
                    // Create the appropriate binary operator node
                    let operator = match token {
                        '|' => Operator::Or,
                        '&' => Operator::And,
                        '^' => Operator::Xor,
                        '=' => Operator::Iff,
                        '>' => Operator::Implies,
                        _ => unreachable!(),
                    };
                    stack.push_back(AstNode::BinaryOperator(operator, Box::new(left), Box::new(right)));
                }
                '!' => {
                    // Ensure there's at least one operand for the unary operator
                    let element = stack.pop_back().ok_or_else(|| "Missing operand for NOT".to_string())?;
                    stack.push_back(AstNode::UnaryOperator(Operator::Not, Box::new(element)));
                }
                _ => return Err(format!("Unknown token: {}", token)),
            }
        }
    
        // Step 2: After the loop, the stack should have exactly one element (the final AST)
        if stack.len() != 1 {
            return Err(format!("Error: The stack should contain exactly one element, but it contains {}", stack.len()));
        }
        
        Ok(stack.pop_back().unwrap()) // Return the AST
    }    
}

impl AstNode {

    pub fn evaluate(&self, vars: &HashMap<char, bool>) -> Result<bool, String> {
        match self {
            AstNode::Variable(var) => {
                match vars.get(var) {
                    Some(value) => Ok(*value),
                    None => Err(format!("Variable '{}' not found", var)),
                }
            },
            
            AstNode::UnaryOperator(op, child) => {
                match op {
                    Operator::Not => {
                        let value = child.evaluate(vars)?;
                        Ok(!value)
                    }
                    _ => panic!("Invalid unary operator"),
                }
            },
            
            AstNode::BinaryOperator(op, left, right) => {
                let left_val = left.evaluate(vars)?;
                let right_val = right.evaluate(vars)?;
                
                match op {
                    Operator::Or        => Ok(left_val | right_val),
                    Operator::And       => Ok(left_val & right_val),
                    Operator::Xor       => Ok(left_val ^ right_val),
                    Operator::Implies   => Ok(!left_val | right_val),
                    Operator::Iff       => Ok(left_val == right_val),
                    _ => panic!("Invalid binary operator"),
                }
            },
        }
    }

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
                        let b = AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf();
                        let a = AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf();
                        
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(a),
                            Box::new(b)
                        )
                    },
                    
                    // De Morgan's laws: ¬(A ∨ B) → (¬A ∧ ¬B)
                    AstNode::BinaryOperator(Operator::Or, left, right) => {
                        let b = AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf();
                        let a = AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf();
                        AstNode::BinaryOperator(
                            Operator::And,
                            Box::new(a),
                            Box::new(b)
                        )
                    },
                    
                    // Handle implication: ¬(A → B) ≡ A ∧ ¬B
                    AstNode::BinaryOperator(Operator::Implies, left, right) => {
                        let b = AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf();
                        let a = left.to_nnf();

                        AstNode::BinaryOperator(
                            Operator::And,
                            Box::new(a),
                            Box::new(b)
                        )
                    },
                    
                    // Handle equivalence: ¬(A ↔ B) ≡ (A ∧ ¬B) ∨ (¬A ∧ B)
                    AstNode::BinaryOperator(Operator::Iff, left, right) => {
                        let b = right.to_nnf();
                        let bi = AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf();
                        let a = left.to_nnf();
                        let ai = AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf();
                        
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(a),
                                Box::new(bi)
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(ai),
                                Box::new(b)
                            ))
                        )
                    },
                    
                    // Handle XOR: ¬(A ⊕ B) ≡ (A ↔ B) ≡ (A ∧ B) ∨ (¬A ∧ ¬B)
                    AstNode::BinaryOperator(Operator::Xor, left, right) => {
                        let b = right.to_nnf();
                        let bi = AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf();
                        let a = left.to_nnf();
                        let ai = AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf();

                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(a),
                                Box::new(b)
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(ai),
                                Box::new(bi)
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
                        let b = right.to_nnf();
                        let a = left.to_nnf();

                        AstNode::BinaryOperator(
                            op.clone(),
                            Box::new(a),
                            Box::new(b)
                        )
                    },
                    
                    // A → B ≡ ¬A ∨ B
                    Operator::Implies => {
                        let b = right.to_nnf();
                        let a = AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf();
                        
                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(a),
                            Box::new(b)
                        )
                    },
                    
                    // A ↔ B ≡ (A ∧ B) ∨ (¬A ∧ ¬B)
                    Operator::Iff => {
                        let b = right.to_nnf();
                        let bi = AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf();
                        let a = left.to_nnf();
                        let ai = AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf();

                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(a),
                                Box::new(b)
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(ai),
                                Box::new(bi)
                            ))
                        )
                    },
                    
                    // A ⊕ B ≡ (A ∧ ¬B) ∨ (¬A ∧ B)
                    Operator::Xor => {
                        let b = right.to_nnf();
                        let bi = AstNode::UnaryOperator(Operator::Not, right.clone()).to_nnf();
                        let a = left.to_nnf();
                        let ai = AstNode::UnaryOperator(Operator::Not, left.clone()).to_nnf();

                        AstNode::BinaryOperator(
                            Operator::Or,
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(a),
                                Box::new(bi)
                            )),
                            Box::new(AstNode::BinaryOperator(
                                Operator::And,
                                Box::new(ai),
                                Box::new(b)
                            ))
                        )
                    },
                    _ => panic!("Error"),
                }
            },
            _ => panic!("Error"),
        }
    }

    pub fn to_rpn(&self) -> String {
        match self {
            AstNode::Variable(var) => String::from(*var),
            
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