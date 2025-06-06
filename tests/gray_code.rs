#[cfg(test)]
mod tests {
    use boole::boole::gray_code;

    #[test]
    fn test_gray_code_0() {
        assert_eq!(gray_code(0), 0)
    }

    #[test]
    fn test_gray_code_1() {
        assert_eq!(gray_code(1), 1)
    }

    #[test]
    fn test_gray_code_2() {
        assert_eq!(gray_code(2), 3)
    }

    #[test]
    fn test_gray_code_3() {
        assert_eq!(gray_code(3), 2)
    }

    #[test]
    fn test_gray_code_4() {
        assert_eq!(gray_code(4), 6)
    }

    #[test]
    fn test_gray_code_5() {
        assert_eq!(gray_code(5), 7)
    }
}
