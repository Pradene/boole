#[cfg(test)]
mod tests {
    use boole::boole::multiplier;

    #[test]
    fn test_multiplier_1() {
        let a = 21;
        let b = 42;

        assert_eq!(multiplier(a, b), a * b)
    }

    #[test]
    fn test_multiplier_2() {
        let a = 90;
        let b = 80;

        assert_eq!(multiplier(a, b), a * b)
    }

    #[test]
    fn test_multiplier_3() {
        let a = 76430;
        let b = 321;

        assert_eq!(multiplier(a, b), a * b)
    }

    #[test]
    fn test_multiplier_4() {
        let a = u32::MAX;
        let b = 1;

        assert_eq!(multiplier(a, b), u32::MAX)
    }
}
