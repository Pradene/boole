#[cfg(test)]
mod tests {
    use boole::boole::adder;

    #[test]
    fn test_zero_operands() {
        assert_eq!(adder(0, 0), 0);
    }

    #[test]
    fn test_identity() {
        assert_eq!(adder(0, u32::MAX), u32::MAX);
        assert_eq!(adder(u32::MAX, 0), u32::MAX);
    }

    #[test]
    fn test_carry_chain() {
        // Trigger multi-bit carry propagation
        let a = 0x00000001;
        let b = 0xFFFFFFFF;
        assert_eq!(adder(a, b), 0);
    }

    #[test]
    fn test_high_bit_overflow() {
        // 2^31 + 2^31 = 2^32 (overflow to 0)
        let a = 0x80000000;
        let b = 0x80000000;
        assert_eq!(adder(a, b), 0);
    }

    #[test]
    fn test_sign_bit_transition() {
        // Max positive + 1 = Min negative (signed perspective)
        let a = 0x7FFFFFFF; // Max positive i32
        let b = 1;
        assert_eq!(adder(a, b), 0x80000000);
    }

    #[test]
    fn test_multi_byte_carries() {
        // Carry propagation across bytes
        let a = 0x0F0F0F0F;
        let b = 0xF0F0F0F0;
        assert_eq!(adder(a, b), u32::MAX);
    }

    #[test]
    fn test_no_carry_operations() {
        // Operations with no carries
        let a = 0x12345678;
        let b = 0x0F0F0F0F;
        assert_eq!(adder(a, b), 0x21436587);
    }
}
