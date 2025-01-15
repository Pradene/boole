pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    let mut mul = b;
    let mut add = a;

    while mul > 0 {
        if mul & 1 == 1 {
            res = adder(res, add);
        }

        add = add << 1;
        mul = mul >> 1;
    }

    res
}

pub fn adder(a: u32, b: u32) -> u32 {
    let mut carry;
    let mut res = a;
    let mut num = b;

    while num != 0 {
        carry = (res & num) << 1;
        res = res ^ num;
        num = carry;
    }

    res
}

pub fn gray_code(a: u32) -> u32 {
    a ^ (a >> 1)
}