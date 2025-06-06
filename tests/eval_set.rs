#[cfg(test)]
mod tests {
    use boole::boole::evaluate_set;

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
