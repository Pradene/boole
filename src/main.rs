use boole::{gray_code};

fn main() {
    let n = 35;

    println!("{:#032b}", n);
    println!("{:#032b}", n >> 1);
    println!("{:#032b}", gray_code(n));
    
    println!("Hello, world!");
}
