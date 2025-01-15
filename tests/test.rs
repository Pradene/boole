use boole::{adder, multiplier};

#[cfg(test)]
mod tests {
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
}