#[cfg(test)]
mod multiplier_tests {
    use boole::boole::multiplier;

    // Basic multiplication
    #[test]
    fn test_basic_multiplication() {
        assert_eq!(multiplier(21, 42), 882);
        assert_eq!(multiplier(90, 80), 7200);
        assert_eq!(multiplier(76430, 321), 76430 * 321);
    }

    // Zero cases
    #[test]
    fn test_zero_operands() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(0, u32::MAX), 0);
        assert_eq!(multiplier(u32::MAX, 0), 0);
    }

    // Identity cases
    #[test]
    fn test_identity() {
        assert_eq!(multiplier(1, u32::MAX), u32::MAX);
        assert_eq!(multiplier(u32::MAX, 1), u32::MAX);
    }

    // Powers of two (shift-based multiplication)
    #[test]
    fn test_powers_of_two() {
        assert_eq!(multiplier(1 << 10, 1 << 5), 1 << 15);
        assert_eq!(multiplier(1 << 31, 1), 1 << 31);
    }

    // Overflow behavior (wrapping around)
    #[test]
    fn test_overflow() {
        assert_eq!(multiplier(1 << 16, 1 << 16), 0); // 2^32 wraps to 0
        assert_eq!(multiplier(u32::MAX, u32::MAX), 1); // (2^32-1)^2 mod 2^32 = 1
    }

    // High-bit multiplication
    #[test]
    fn test_high_bit_multiplication() {
        assert_eq!(multiplier(0x80000000, 2), 0); // 2^31 × 2 = 2^32 → 0 (overflow)
    }

    // Commutativity check
    #[test]
    fn test_commutativity() {
        let x = 123456789;
        let y = 987654321;
        assert_eq!(multiplier(x, y), multiplier(y, x));
    }
}
