use boole::boole::{map, reverse_map};

fn main() {
    let mapped = map(165, u16::MAX);
    println!("result : {}", mapped);

    let original = reverse_map(mapped);
    println!("result : {:?}", original);
}
