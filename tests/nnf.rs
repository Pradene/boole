mod tests {
    use boole::boole::negation_normal_form;

    #[test]
    fn test_nnf1() {
        let nnf = negation_normal_form("AB&!");
        let result = "A!B!|";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_nnf2() {
        let nnf = negation_normal_form("AB|!");
        let result = "A!B!&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_nnf3() {
        let nnf = negation_normal_form("AB|C&!");
        let result = "A!B!&C!|";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_nnf4() {
        let nnf = negation_normal_form("AB=");
        let result = "AB&A!B!&|";
        assert_eq!(result, nnf)
    }
}
