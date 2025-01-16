use boole::ast::AstNode;

fn main() {
    let formula = "10&1|1>!1|";

    match AstNode::try_from(formula) {
        Ok(ast) => {
            // Print the AST as a string using the Display trait
            println!("AST: {}", ast);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
