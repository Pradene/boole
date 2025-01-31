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
    // Get the truth table from the existing method
    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let truth_table = ast.truth_table();

    // Check if the truth table is empty
    if truth_table.is_empty() {
        println!("No truth table to print.");
        return;
    }

    // Get the variables from the truth table
    let var_list: Vec<char> = ast.get_variables().into_iter().collect();

    // Print the header
    for var in &var_list {
        print!("| {} ", var);
    }
    println!("| = |");

    // Print the separator line
    for _ in &var_list {
        print!("|---");
    }
    println!("|---|");

    // Print each row of the truth table
    for (values, result) in truth_table {
        for var in &var_list {
            print!("| {} ", if *values.get(var).unwrap() { 1 } else { 0 });
        }
        println!("| {} |", if result { 1 } else { 0 });
    }
}


pub fn negation_normal_form(formula: &str) -> String {
    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let nnf = ast.to_nnf();

    nnf.to_rpn()
}


pub fn conjunctive_normal_form(formula: &str) -> String {
    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let nnf = ast.to_cnf();

    nnf.to_rpn()
}


pub fn sat(formula: &str) -> bool {
    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let truth_table = ast.truth_table();

    // Check if the truth table is empty
    if truth_table.is_empty() {
        return false;
    }

    // Print each row of the truth table
    for (values, result) in truth_table {
        if result {
            return true;
        }
    }

    false
}