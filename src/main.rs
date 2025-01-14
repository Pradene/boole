fn adder(a: u32, b: u32) -> u32 {
    let mut carry;
    let mut result = a;
    let mut num = b;

    println!("a: {:#032b}", result);
    println!("b: {:#032b}", num);
    
    while num != 0 {
        
        carry = (result & num) << 1;
        result = result ^ num;
        num = carry;
    }
    
    println!("a: {:#032b}", result);
    println!("");

    result
}

fn main() {

    adder(3, 4);
    adder(4, 4);
    adder(90, 87);

    println!("Hello, world!");
}
