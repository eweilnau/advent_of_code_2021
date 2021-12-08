pub fn print_answers() {
    println!("\n--- Day 1: Sonar Sweep ---");
    let sonar = Sonar::new(&std::fs::read_to_string("assets\\day_01_input.txt").unwrap());
    println!(
        "How many measurements are larger than the previous measurement? {}",
        sonar.count_depth_measurement_increases(),
    );
    println!(
        "Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum? {}",
        sonar.count_depth_sliding_sum_increases(3),
    );
}

struct Sonar {
    depth_measurements: Vec<u16>,
}

impl Sonar {
    fn new(input: &str) -> Self {
        let depth_measurements = input
            .lines()
            .map(|line| line.parse::<u16>().unwrap())
            .collect::<Vec<_>>();
        Self { depth_measurements }
    }

    fn count_depth_measurement_increases(&self) -> usize {
        let mut results = 0;
        for i in 1..self.depth_measurements.len() {
            if self.depth_measurements[i - 1] < self.depth_measurements[i] {
                results += 1;
            }
        }
        results
    }

    fn count_depth_sliding_sum_increases(&self, sliding_window_size: usize) -> usize {
        let mut results = 0;
        for i in 1..self.depth_measurements.len() - (sliding_window_size - 1) {
            let previous_sum = self.depth_measurements[i - 1..i + (sliding_window_size - 1)]
                .iter()
                .sum::<u16>();
            let current_sum = self.depth_measurements[i..i + sliding_window_size]
                .iter()
                .sum::<u16>();
            if previous_sum < current_sum {
                results += 1
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_count_depth_measurement_increases() {
        let sonar = Sonar::new(INPUT);
        assert_eq!(sonar.count_depth_measurement_increases(), 7);
    }

    #[test]
    fn test_count_depth_sliding_sum_increases() {
        let sonar = Sonar::new(INPUT);
        assert_eq!(sonar.count_depth_sliding_sum_increases(3), 5);
    }
}
