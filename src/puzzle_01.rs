pub fn one_a(mass: i32) -> i32 {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_a_value_12() {
        assert_eq!(2, one_a(12));
    }
    #[test]
    fn test_one_a_value_14() {
        assert_eq!(2, one_a(14));
    }
    #[test]
    fn test_one_a_value_654() {
        assert_eq!(654, one_a(1969));
    }
    #[test]
    fn test_one_a_value_100756() {
        assert_eq!(33583, one_a(100756));
    }
}
