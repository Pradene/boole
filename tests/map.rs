#[cfg(test)]
mod tests {
    use boole::boole::{map, reverse_map};

    #[test]
    pub fn test_map_0() {
        let a = 0;
        let b = 0;

        assert_eq!(map(a, b), 0.0);
    }

    #[test]
    pub fn test_map_1() {
        let a = 10;
        let b = 100;

        assert_eq!(reverse_map(map(a, b)), (a, b));
    }

    #[test]
    pub fn test_map_2() {
        let a = u16::MAX;
        let b = u16::MAX;

        assert_eq!(reverse_map(map(a, b)), (a, b));
    }
}
