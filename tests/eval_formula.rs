#[cfg(test)]
mod tests {
    use boole::boole::eval_formula;

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
}
