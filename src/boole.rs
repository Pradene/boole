use std::collections::{LinkedList, HashSet, HashMap};

use crate::ast::AstNode;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    let mut mul = b;
    let mut add = a;

    while mul > 0 {
        if mul & 1 == 1 {
            res = adder(res, add);
        }

        add = add << 1;
        mul = mul >> 1;
    }

    res
}


pub fn adder(a: u32, b: u32) -> u32 {
    let mut carry;
    let mut res = a;
    let mut num = b;

    while num != 0 {
        carry = (res & num) << 1;
        res = res ^ num;
        num = carry;
    }

    res
}


pub fn gray_code(a: u32) -> u32 {
    a ^ (a >> 1)
}


pub fn eval_formula(formula: &str) -> Result<bool, String> {
    let mut stack: LinkedList<bool> = LinkedList::new();

    // Helper function to evaluate binary operations
    fn eval_binary_op<F>(op: F, stack: &mut LinkedList<bool>) -> Result<(), String>
    where F: Fn(bool, bool) -> bool {
        if let (Some(b), Some(a)) = (stack.pop_back(), stack.pop_back()) {
            stack.push_back(op(a, b));
            Ok(())
        } else {
            Err("Not enough values in stack for operator".to_string())
        }
    }

    // Helper function to evaluate unary operations
    fn eval_unary_op<F>(op: F, stack: &mut LinkedList<bool>) -> Result<(), String>
    where F: Fn(bool) -> bool {
        if let Some(a) = stack.pop_back() {
            stack.push_back(op(a));
            Ok(())
        } else {
            Err("Not enough values in stack for unary operator".to_string())
        }
    }

    // Iterate over the formula characters and process each one
    for ch in formula.chars() {
        match ch {
            '0' => stack.push_back(false),
            '1' => stack.push_back(true),
            '|' => eval_binary_op(|a, b| a | b, &mut stack)?,
            '&' => eval_binary_op(|a, b| a & b, &mut stack)?,
            '^' => eval_binary_op(|a, b| a ^ b, &mut stack)?,
            '>' => eval_binary_op(|a, b| !a | b, &mut stack)?,
            '=' => eval_binary_op(|a, b| a == b, &mut stack)?,
            '!' => eval_unary_op(|a| !a, &mut stack)?,
            _ => return Err(format!("Invalid character encountered: {}", ch)),
        }
    }

    // Final stack check
    if stack.len() == 1 {
        Ok(stack.pop_back().unwrap())
    } else {
        Err("Invalid formula: stack has more than one value or is empty".to_string())
    }
}


pub fn print_truth_table(formula: &str) {

    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let mut truth_table = format!("");
    
    let unique_variables: HashSet<_> = 
        formula.chars()
        .filter(|&c| c.is_alphabetic())
        .collect();
    let n = unique_variables.len();
    
    for var in unique_variables.iter() {
        truth_table.push_str(&format!("| {} ", var));
    }
    truth_table.push_str(&format!("| = |\n"));
    
    for _ in 0..n {
        truth_table.push_str(&format!("|---"));
    }
    truth_table.push_str(&format!("|---|\n"));
    
    // Generate all combinations of truth values (0 or 1) for n variables
    for i in 0..(1 << n) {
        let mut values: HashMap<char, bool> = HashMap::new();
        for (index, variable) in unique_variables.clone().into_iter().enumerate() {
            let value = (i >> ((n - 1) - index)) & 1 == 1;
            truth_table.push_str(&format!("| {:?} ", if value {1} else {0}));
            values.insert(variable, value);
        }

        truth_table.push_str(&format!("| {} |\n", if ast.evaluate(&values).expect("Couldn't evaluate formula") {1} else {0}));
    }

    print!("{}", truth_table);
}


pub fn negation_normal_form(formula: &str) -> String {
    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let nnf = ast.to_nnf();

    nnf.to_rpn()
}