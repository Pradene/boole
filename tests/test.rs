use boole::boole::{
    adder, conjunctive_normal_form, eval_formula, evaluate_set, gray_code, multiplier,
    negation_normal_form, sat,
};

#[cfg(test)]
mod tests {
    use boole::boole::powerset;

    use super::*;

    #[test]
    fn test_adder1() {
        let a = 21;
        let b = 42;

        assert_eq!(a + b, adder(a, b))
    }

    #[test]
    fn test_adder2() {
        let a = 90;
        let b = 80;

        assert_eq!(a + b, adder(a, b))
    }

    #[test]
    fn test_adder3() {
        let a = 76430;
        let b = 43434345;

        assert_eq!(a + b, adder(a, b))
    }

    #[test]
    fn test_adder4() {
        let a = u32::MAX - 1;
        let b = 1;

        assert_eq!(a + b, adder(a, b))
    }

    #[test]
    fn test_multiplier1() {
        let a = 21;
        let b = 42;

        assert_eq!(a * b, multiplier(a, b))
    }

    #[test]
    fn test_multiplier2() {
        let a = 90;
        let b = 80;

        assert_eq!(a * b, multiplier(a, b))
    }

    #[test]
    fn test_multiplier3() {
        let a = 76430;
        let b = 321;

        assert_eq!(a * b, multiplier(a, b))
    }

    #[test]
    fn test_multiplier4() {
        let a = u32::MAX - 1;
        let b = 1;

        assert_eq!(a * b, multiplier(a, b))
    }

    #[test]
    fn test_gray_code1() {
        assert_eq!(0, gray_code(0))
    }

    #[test]
    fn test_gray_code2() {
        assert_eq!(3, gray_code(2))
    }

    #[test]
    fn test_gray_code3() {
        assert_eq!(12, gray_code(8))
    }

    #[test]
    fn test_gray_code4() {
        assert_eq!(7, gray_code(5))
    }

    #[test]
    fn test_eval_formula1() {
        assert_eq!(true, eval_formula("10|").unwrap())
    }

    #[test]
    fn test_eval_formula2() {
        assert_eq!(false, eval_formula("10&").unwrap())
    }

    #[test]
    fn test_eval_formula3() {
        assert_eq!(true, eval_formula("1011||=").unwrap())
    }

    #[test]
    fn test_eval_formula4() {
        assert_eq!(true, eval_formula("10|1=").unwrap())
    }

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

    #[test]
    fn test_sat1() {
        let result = sat("AB|");
        assert_eq!(true, result)
    }

    #[test]
    fn test_sat2() {
        let result = sat("AB&");
        assert_eq!(true, result)
    }

    #[test]
    fn test_sat3() {
        let result = sat("AA!&");
        assert_eq!(false, result)
    }

    #[test]
    fn test_sat4() {
        let result = sat("AA^");
        assert_eq!(false, result)
    }

    #[test]
    fn test_powerset1() {
        let set = vec![1];
        let result = vec![vec![], vec![1]];

        assert_eq!(powerset(set), result);
    }

    #[test]
    fn test_powerset2() {
        let set = vec![0, 1];
        let result = vec![vec![], vec![0], vec![1], vec![0, 1]];

        assert_eq!(powerset(set), result);
    }

    #[test]
    fn test_not_operator() {
        // NOT A where A = [1,2] and universal set is [1,2,3,4]
        let formula = "A!";
        let sets = vec![vec![1, 2], vec![3, 4]]; // A, B (B ignored here)
        assert_eq!(evaluate_set(formula, sets), vec![3, 4]);
    }

    #[test]
    fn test_and_operator() {
        // A AND B where A = [1,2], B = [2,3]
        let formula = "AB&";
        let sets = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(evaluate_set(formula, sets), vec![2]);
    }

    #[test]
    fn test_or_operator() {
        // A OR B where A = [1,2], B = [2,3]
        let formula = "AB|";
        let sets = vec![vec![1, 2], vec![2, 3]];
        let mut result = evaluate_set(formula, sets);
        result.sort_unstable();
        assert_eq!(result, vec![1, 2, 3]);
    }
}
