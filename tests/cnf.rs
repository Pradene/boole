mod tests {
    use boole::boole::conjunctive_normal_form;

    #[test]
    fn test_cnf_0() {
        let nnf = conjunctive_normal_form("AB&!");
        let result = "A!B!|";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf_1() {
        let nnf = conjunctive_normal_form("AB|!");
        let result = "A!B!&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf_2() {
        let nnf = conjunctive_normal_form("AB|C&");
        let result = "AB|C&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf_3() {
        let nnf = conjunctive_normal_form("AB|C|D|");
        let result = "ABCD|||";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf_4() {
        let nnf = conjunctive_normal_form("AB&C&D&");
        let result = "ABCD&&&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf_5() {
        let nnf = conjunctive_normal_form("AB&!C!|");
        let result = "A!B!C!||";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf_6() {
        let nnf = conjunctive_normal_form("AB|!C!&");
        let result = "A!B!C!&&";
        assert_eq!(result, nnf)
    }

    #[test]
    fn test_cnf_7() {
        let nnf = conjunctive_normal_form("ABCD&|&");
        let result = "ABC|BD|&&";
        assert_eq!(result, nnf)
    }
}
