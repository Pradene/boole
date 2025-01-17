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

    let var = [false; 26];

    println!("{}", ast.evaluate(&var));
    println!("{}", eval_formula(formula));

}
