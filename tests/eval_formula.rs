#[cfg(test)]
mod tests {
    use boole::boole::eval_formula;

    #[test]
    fn test_eval_formula_1() {
        assert_eq!(true, eval_formula("10|").unwrap())
    }

    #[test]
    fn test_eval_formula_2() {
        assert_eq!(false, eval_formula("10&").unwrap())
    }

    #[test]
    fn test_eval_formula_3() {
        assert_eq!(true, eval_formula("1011||=").unwrap())
    }

    #[test]
    fn test_eval_formula_4() {
        assert_eq!(true, eval_formula("10|1=").unwrap())
    }
}
