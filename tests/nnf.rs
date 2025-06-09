mod tests {
    use boole::boole::negation_normal_form;

    #[test]
    fn test_nnf_0() {
        let nnf = negation_normal_form("AB&!");
        let result = "A!B!|";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_nnf_1() {
        let nnf = negation_normal_form("AB|!");
        let result = "A!B!&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_nnf_2() {
        let nnf = negation_normal_form("AB>");
        let result = "A!B|";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_nnf_3() {
        let nnf = negation_normal_form("AB=");
        let result = "AB&A!B!&|";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_nnf_4() {
        let nnf = negation_normal_form("AB|C&!");
        let result = "A!B!&C!|";
        assert_eq!(result, nnf)
    }
}
