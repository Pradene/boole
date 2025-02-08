use boole::boole::evaluate_set;

fn main() {
    let sets = vec![vec![0, 1, 2, 3], vec![3, 4, 5, 6]];

    let formula = "AB&!";
    let result = evaluate_set(formula, sets);

    println!("result : {:?}", result);
}
