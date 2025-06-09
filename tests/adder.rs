#[cfg(test)]
mod tests {
    use boole::boole::adder;

    #[test]
    fn test_adder_1() {
        let a = 21;
        let b = 42;

        assert_eq!(a + b, adder(a, b));
    }

    #[test]
    fn test_adder_2() {
        let a = 90;
        let b = 80;

        assert_eq!(a + b, adder(a, b));
    }

    #[test]
    fn test_adder_3() {
        let a = 76430;
        let b = 43434345;

        assert_eq!(a + b, adder(a, b));
    }

    #[test]
    fn test_adder_4() {
        let a = u32::MAX - 1;
        let b = 1;

        assert_eq!(adder(a, b), u32::MAX);
    }

    #[test]
    fn test_adder_5() {
        let a = u32::MAX;
        let b = 1;

        assert_eq!(adder(a, b), u32::MIN);
    }
}
