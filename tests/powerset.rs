#[cfg(test)]
mod tests {
    use boole::boole::powerset;

    #[test]
    fn test_powerset_1() {
        let set = vec![1];
        let result = vec![vec![], vec![1]];

        assert_eq!(powerset(set), result);
    }

    #[test]
    fn test_powerset_2() {
        let set = vec![0, 1];
        let result = vec![vec![], vec![0], vec![1], vec![0, 1]];

        assert_eq!(powerset(set), result);
    }
}
