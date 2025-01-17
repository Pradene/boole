use std::collections::HashMap;

use boole::ast::AstNode;
use boole::boole::{negation_normal_form, eval_formula, print_truth_table};

fn main() {
    let formula = "AB>";

    print_truth_table(formula);

    let ast = AstNode::try_from(formula).expect("Can't create AST from formula");
    println!("{}", ast);
    println!("{}", ast.to_string());

    let nnf = ast.to_nnf();
    println!("{}", nnf);
    println!("{}", nnf.to_rpn());

    println!("{}", negation_normal_form(formula));

    let mut variables: HashMap<char, bool> = HashMap::new();
    variables.insert('A', false);
    variables.insert('B', true);

    println!("{}", ast.evaluate(&variables).expect("Couldn't evaluate formula, because variable not found"));
    // println!("{}", eval_formula(formula).expect("Couldn't evaluate formula"));

    let f = "AB|!C!&";
    let a = AstNode::try_from(f).expect("Couldn't create AST");
    let cnf = a.to_cnf();
    println!("{}", cnf.to_rpn());
    println!("{}", cnf);

}
