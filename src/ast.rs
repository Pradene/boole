use std::collections::{
    LinkedList,
    HashMap,
    HashSet
};
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

    pub fn get_variables(&self) -> HashSet<char> {
        let mut variables = HashSet::new();
        match self {
            AstNode::Variable(c) => {
                variables.insert(*c);
            }
            AstNode::BinaryOperator(_, left, right) => {
                variables.extend(left.get_variables());
                variables.extend(right.get_variables());
            }
            AstNode::UnaryOperator(_, child) => {
                variables.extend(child.get_variables());
            }
        }
        
        variables
    }

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

    pub fn truth_table(&self) -> Vec<(HashMap<char, bool>, bool)> {
        let variables = self.get_variables();
        let num_vars = variables.len();

        let mut truth_table = Vec::new();
        let var_list: Vec<char> = variables.into_iter().collect();

        // There are 2^n possible truth assignments for n variables
        for i in 0..(1 << num_vars) {
            let mut values = HashMap::new();
            for (j, &var) in var_list.iter().enumerate() {
                // Assign true/false based on the current bit in the integer i
                values.insert(var, (i >> j) & 1 == 1);
            }

            // Step 3: Evaluate the AST with the current variable assignments
            let result = self.evaluate(&values).unwrap();

            // Store the combination of variable assignments and the result
            truth_table.push((values, result));
        }

        truth_table
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

    pub fn to_cnf(&self) -> AstNode {
        // First convert to NNF then apply CNF conversion
        fn distribute(node: &AstNode) -> AstNode {
            match node {
                // Base cases
                AstNode::Variable(_) | AstNode::UnaryOperator(_, _) => node.clone(),

                AstNode::BinaryOperator(op, left, right) => match op {
                    // AND: recursively convert both sides
                    Operator::And => AstNode::BinaryOperator(
                        Operator::And,
                        Box::new(distribute(left)),
                        Box::new(distribute(right)),
                    ),

                    // OR: need to handle distribution
                    Operator::Or => {
                        let left_cnf = distribute(left);
                        let right_cnf = distribute(right);

                        match (&left_cnf, &right_cnf) {
                            // Case: (A ∧ B) ∨ C -> (A ∨ C) ∧ (B ∨ C)
                            (AstNode::BinaryOperator(Operator::And, a, b), c) => distribute(
                                &AstNode::BinaryOperator(
                                    Operator::And,
                                    Box::new(AstNode::BinaryOperator(
                                        Operator::Or,
                                        a.clone(),
                                        Box::new(c.clone()),
                                    )),
                                    Box::new(AstNode::BinaryOperator(
                                        Operator::Or,
                                        b.clone(),
                                        Box::new(c.clone()),
                                    )),
                                ),
                            ),

                            // Case: A ∨ (B ∧ C) -> (A ∨ B) ∧ (A ∨ C)
                            (a, AstNode::BinaryOperator(Operator::And, b, c)) => distribute(
                                &AstNode::BinaryOperator(
                                    Operator::And,
                                    Box::new(AstNode::BinaryOperator(
                                        Operator::Or,
                                        Box::new(a.clone()),
                                        b.clone(),
                                    )),
                                    Box::new(AstNode::BinaryOperator(
                                        Operator::Or,
                                        Box::new(a.clone()),
                                        c.clone(),
                                    )),
                                ),
                            ),

                            // No AND to distribute over
                            _ => AstNode::BinaryOperator(
                                Operator::Or,
                                Box::new(left_cnf),
                                Box::new(right_cnf),
                            ),
                        }
                    }
                    _ => panic!("Unexpected operator in CNF conversion"),
                },
                _ => panic!("Unexpected node type"),
            }
        }
        
        distribute(&self.to_nnf()).build_right_leaning_tree()
    }
    
    fn build_right_leaning_tree(&self) -> AstNode {
        fn collect_clauses(node: &AstNode) -> Vec<AstNode> {
            match node {
                // If it's an AND, collect clauses from both sides
                AstNode::BinaryOperator(Operator::And, left, right) => {
                    let mut clauses = Vec::<AstNode>::new();
                    clauses.extend(collect_clauses(left));
                    clauses.extend(collect_clauses(right));
                    clauses
                }
                // If it's not an AND, it's a single clause
                _ => vec![node.clone()],
            }
        }

        let mut iter = collect_clauses(self).into_iter().rev();
        let last = iter.next().expect("CNF must have at least one clause");
        
        iter.fold(last, |acc, clause| {
            AstNode::BinaryOperator(
                Operator::And,
                Box::new(clause),
                Box::new(acc),
            )
        })
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