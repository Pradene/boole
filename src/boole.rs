use std::collections::{HashSet, LinkedList};

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

    return res;
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

    return res;
}

pub fn gray_code(a: u32) -> u32 {
    return a ^ (a >> 1);
}

pub fn eval_formula(formula: &str) -> Result<bool, String> {
    let mut stack: LinkedList<bool> = LinkedList::new();

    // Helper function to evaluate binary operations
    fn eval_binary_op<F>(op: F, stack: &mut LinkedList<bool>) -> Result<(), String>
    where
        F: Fn(bool, bool) -> bool,
    {
        if let (Some(b), Some(a)) = (stack.pop_back(), stack.pop_back()) {
            stack.push_back(op(a, b));
            Ok(())
        } else {
            Err("Not enough values in stack for operator".to_string())
        }
    }

    // Helper function to evaluate unary operations
    fn eval_unary_op<F>(op: F, stack: &mut LinkedList<bool>) -> Result<(), String>
    where
        F: Fn(bool) -> bool,
    {
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
        return Ok(stack.pop_back().unwrap());
    } else {
        return Err("Invalid formula: stack has more than one value or is empty".to_string());
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

    return nnf.to_rpn();
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let nnf = ast.to_cnf();

    return nnf.to_rpn();
}

pub fn sat(formula: &str) -> bool {
    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    let truth_table = ast.truth_table();

    // Check if the truth table is empty
    if truth_table.is_empty() {
        return false;
    }

    // Print each row of the truth table
    for (_, result) in truth_table {
        if result {
            return true;
        }
    }

    return false;
}

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    let possibility = 1 << set.len();
    for i in 0..possibility {
        let mut v = Vec::new();

        for (index, value) in set.iter().enumerate() {
            if i & (1 << index) != 0 {
                v.push(*value);
            }
        }

        res.push(v);
    }

    return res;
}

pub fn evaluate_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let ast = AstNode::try_from(formula).unwrap();
    let universal_set: HashSet<i32> = sets.iter().flatten().cloned().collect();

    return ast.evaluate_set(sets, universal_set).unwrap();
}

pub fn map(x: u16, y: u16) -> f64 {
    let mut result: u32 = 0;

    for i in 0..16 {
        let x_bit = (x >> i) & 1;
        let y_bit = (y >> i) & 1;

        result |= ((x_bit as u32) << (2 * i)) | ((y_bit as u32) << (2 * i + 1));
    }

    return result as f64 / u32::MAX as f64;
}

pub fn reverse_map(z: f64) -> (u16, u16) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;

    let z = (z * u32::MAX as f64) as u32;

    for i in 0..16 {
        x |= (((z >> (2 * i)) & 1) << i) as u16; // Extract every 2nd bit starting from 0
        y |= (((z >> (2 * i + 1)) & 1) << i) as u16; // Extract every 2nd bit starting from 1
    }

    return (x, y);
}
