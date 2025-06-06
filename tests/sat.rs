#[cfg(test)]
mod tests {
    use boole::boole::sat;

    #[test]
    fn test_sat_1() {
        let result = sat("AB|");
        assert_eq!(true, result)
    }

    #[test]
    fn test_sat_2() {
        let result = sat("AB&");
        assert_eq!(true, result)
    }

    #[test]
    fn test_sat_3() {
        let result = sat("AA!&");
        assert_eq!(false, result)
    }

    #[test]
    fn test_sat_4() {
        let result = sat("AA^");
        assert_eq!(false, result)
    }
}
