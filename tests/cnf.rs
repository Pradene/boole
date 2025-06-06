mod tests {
    use boole::boole::conjunctive_normal_form;

    #[test]
    fn test_cnf1() {
        let nnf = conjunctive_normal_form("AB&!");
        let result = "A!B!|";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf2() {
        let nnf = conjunctive_normal_form("AB|!");
        let result = "A!B!&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf3() {
        let nnf = conjunctive_normal_form("AB&C&D&");
        let result = "ABCD&&&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf4() {
        let nnf = conjunctive_normal_form("AB|!C!&");
        let result = "A!B!C!&&";
        assert_eq!(result, nnf)
    }
}
