#[cfg(test)]
mod tests {
    use boole::boole::evaluate_set;

    #[test]
    fn test_not_operator() {
        let formula = "A!";
        let sets = vec![vec![0, 1, 2]];
        assert_eq!(evaluate_set(formula, sets), vec![]);
    }

    #[test]
    fn test_and_operator() {
        let formula = "AB&";
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
        assert_eq!(evaluate_set(formula, sets), vec![0]);
    }

    #[test]
    fn test_or_operator() {
        let formula = "AB|";
        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let result = evaluate_set(formula, sets);
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_equal() {
        let formula = "AB=";
        let sets = vec![vec![0, 1, 2], vec![0, 1, 2]];
        let result = evaluate_set(formula, sets);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_not_equal() {
        let formula = "AB=";
        let sets = vec![vec![0, 1, 2], vec![0, 1, 3]];
        let result = evaluate_set(formula, sets);
        assert_eq!(result, vec![0, 1]);
    }
}
