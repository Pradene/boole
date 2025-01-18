use std::collections::HashMap;

use boole::ast::AstNode;
use boole::boole::{
    negation_normal_form,
    eval_formula,
    truth_table
};

fn main() {

    // let f = "ABCD&|&";
    let f = "AB|!C!&";
    // let f = "AB=";
    // let f = "AB|!";
    // let f = "AB|C&";
    // let f = "AB&C&D&";
    // let f = "AB&C&D|";

    let a = AstNode::try_from(f).expect("Couldn't create AST");
    println!("{}", a);
    println!("{}", a.to_cnf().to_rpn());
    // println!("{}", a.to_cnf().to_rpn());
    
    // let ast = a.to_nnf();
}
