use boole::boole::print_truth_table;

fn main() {
    println!("Truth table of & :");
    print_truth_table("AB&");
    println!();

    println!("Truth table of | :");
    print_truth_table("AB|");
    println!();

    println!("Truth table of > :");
    print_truth_table("AB>");
    println!();
}
