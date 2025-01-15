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

pub fn eval_formula(formula: &str) -> bool {
    let mut stack: LinkedList<u8> = LinkedList::new();

    for ch in formula.chars() {
        match ch {
            '0' => {
                stack.push_back(0);
            },
            '1' => {
                stack.push_back(1);
            },
            '|' => {
                if let (Some(a), Some(b)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(a | b);
                } else {
                    return false;
                }
            }, 
            '&' => {
                if let (Some(a), Some(b)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(a & b);
                } else {
                    return false;
                }
            },
            '^' => {
                if let (Some(a), Some(b)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back(a ^ b);
                } else {
                    return false;
                }
            }, 
            '>' => {
                if let (Some(a), Some(b)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back((a >= b) as u8);
                } else {
                    return false;
                }
            },
            '=' => {
                if let (Some(a), Some(b)) = (stack.pop_back(), stack.pop_back()) {
                    stack.push_back((a == b) as u8);
                } else {
                    return false;
                }
            }, 
            '!' => {
                if let Some(a) = stack.pop_back() {
                    stack.push_back(!a);
                } else {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }

    stack.len() == 1 && stack.pop_back().unwrap() == 1
}