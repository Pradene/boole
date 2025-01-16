use std::collections::LinkedList;

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


// Need to display error message if format is incorrect
pub fn eval_formula(formula: &str) -> bool {
    let mut stack: LinkedList<bool> = LinkedList::new();

    for ch in formula.chars() {
        match ch {
            '0' => stack.push_back(false),
            '1' => stack.push_back(true),
            '|' => {
                if let (Some(b), Some(a)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(a | b);
                } else {
                    return false;
                }
            }
            '&' => {
                if let (Some(b), Some(a)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(a & b);
                } else {
                    return false;
                }
            }
            '^' => {
                if let (Some(b), Some(a)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(a ^ b);
                } else {
                    return false;
                }
            }
            '>' => {
                if let (Some(b), Some(a)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(!a | b);
                } else {
                    return false;
                }
            }
            '=' => {
                if let (Some(b), Some(a)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(a == b);
                } else {
                    return false;
                }
            }
            '!' => {
                if let Some(a) = stack.pop_back() {
                    stack.push_back(!a);
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.len() == 1 && stack.pop_back().unwrap() == true
}


pub fn print_truth_table(formula: &str) {
    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZ!&|^>=";
    // Checking if formula contains a wrong character
    if formula.chars().filter(| &c | symbols.contains(c)).count() != formula.len() {
        return;
    }

    let get_unique_variables = || {
        let mut variables: Vec<_> =
            formula.chars()
            .filter(|&c| c.is_alphabetic())
            .collect();
        variables.sort();
        variables.dedup();

        variables
    };

    let unique_variables = get_unique_variables();
    let n = unique_variables.len();

    let mut truth_table = format!("");
    
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
        let mut combination = Vec::new();
        let mut f = formula.to_string();

        for (j, var) in unique_variables.iter().enumerate() {
            combination.push((i >> ((n - 1) - j)) & 1);
            let value = combination[j];
            
            f = f.replace(&var.to_string(), &value.to_string());
            
            truth_table.push_str(&format!("| {:?} ", value));
        }

        truth_table.push_str(&format!("| {} |\n", if eval_formula(&f) {1} else {0}));
    }

    print!("{}", truth_table);
}


// pub fn negation_normal_form(formula: &str) -> String {

// }