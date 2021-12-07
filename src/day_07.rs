pub fn print_answers() {
    println!("\n--- Day 7: The Treachery of Whales ---");
    let crabs = CrabFleet::new(&std::fs::read_to_string("assets\\day_7_input.txt").unwrap());
    println!(
        "Using constant fuel rate, determine the position with the cheapest fuel cost. How much fuel must they spend to align to that position? {}",
        crabs.get_cheapest_constant_fuel_cost()
    );
    println!(
        "Using actual fuel rate, determine the position with the cheapest fuel cost. How much fuel must they spend to align to that position {}",
        crabs.get_cheapest_fuel_cost()
    );
}

struct CrabFleet {
    positions: Vec<i32>,
}

impl CrabFleet {
    fn new(input: &str) -> Self {
        let positions = input
            .trim()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        Self { positions }
    }

    fn calculate_fuel_cost(&self, destination: i32) -> i32 {
        self.positions
            .iter()
            .map(|p| (0..=(p - destination).abs()).sum::<i32>())
            .sum()
    }

    fn get_cheapest_fuel_cost(&self) -> i32 {
        let mean = self.get_mean_position();
        let median = self.get_median_position();
        (std::cmp::min(mean, median)..=std::cmp::max(mean, median))
            .map(|p| self.calculate_fuel_cost(p))
            .min()
            .unwrap()
    }

    fn calculate_constant_fuel_cost(&self, destination: i32) -> i32 {
        self.positions.iter().map(|p| (p - destination).abs()).sum()
    }

    fn get_cheapest_constant_fuel_cost(&self) -> i32 {
        let mean = self.get_mean_position();
        let median = self.get_median_position();
        (std::cmp::min(mean, median)..=std::cmp::max(mean, median))
            .map(|p| self.calculate_constant_fuel_cost(p))
            .min()
            .unwrap()
    }

    fn get_median_position(&self) -> i32 {
        let mut positions = self.positions.clone();
        positions.sort();
        positions[positions.len() / 2]
    }

    fn get_mean_position(&self) -> i32 {
        (self.positions.iter().sum::<i32>() as f32 / self.positions.len() as f32).round() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_get_cheapest_fuel_cost() {
        let crabs = CrabFleet::new(INPUT);
        assert_eq!(crabs.get_cheapest_fuel_cost(), 168);
    }

    #[test]
    fn test_get_cheapest_constant_fuel_cost() {
        let crabs = CrabFleet::new(INPUT);
        assert_eq!(crabs.get_cheapest_constant_fuel_cost(), 37);
    }
}
