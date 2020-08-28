pub fn calculate_fuel_for_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}

pub fn calculate_fuel_for_fuel(mass: i32, fuel: i32) -> i32 {
    if mass <= 0 { return fuel; }
    let extra_fuel: i32 = calculate_fuel_for_mass(mass);
    return calculate_fuel_for_fuel(extra_fuel, extra_fuel + fuel);
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Puzzle 01 A tests
    #[test]
    fn test_one_a_value_12() {
        assert_eq!(2, calculate_fuel_for_mass(12));
    }
    #[test]
    fn test_one_a_value_14() {
        assert_eq!(2, calculate_fuel_for_mass(14));
    }
    #[test]
    fn test_one_a_value_654() {
        assert_eq!(654, calculate_fuel_for_mass(1969));
    }
    #[test]
    fn test_one_a_value_100756() {
        assert_eq!(33583, calculate_fuel_for_mass(100756));
    }

    /// Puzzle 01 B tests
    #[test]
    fn test_one_b_value_14() {
        assert_eq!(0, calculate_fuel_for_fuel(14, 0));
    }

}
