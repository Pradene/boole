use boole::ast::AstNode;
use boole::boole::negation_normal_form;

fn main() {
    let formula = "10=";

    let ast =  AstNode::try_from(formula).expect("Can't create AST from formula");
    println!("{}", ast);
    println!("{}", ast.to_string());
    
    let nnf = ast.to_nnf();
    println!("{}", nnf);
    println!("{}", nnf.to_rpn());

    println!("{}", negation_normal_form(formula));

}
