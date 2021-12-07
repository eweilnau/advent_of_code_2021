pub fn print_answers() {
    println!("\n--- Day 6: Lanternfish ---");
    let mut population =
        Population::new(&std::fs::read_to_string("assets\\day_6_input.txt").unwrap());
    println!(
        "How many lanterfish would there be after 80 days? {}",
        population.simulate(80)
    );
    let mut population =
        Population::new(&std::fs::read_to_string("assets\\day_6_input.txt").unwrap());
    println!(
        "How many lanterfish would there be after 256 days? {}",
        population.simulate(256)
    );
}

struct Population {
    adult_fish: Vec<u128>,
    baby_fish: Vec<u128>,
    threshold: usize,
    spawning: usize,
}

impl Default for Population {
    fn default() -> Self {
        let adult_fish = vec![0; 7];
        let baby_fish = vec![0; 2];
        Self {
            adult_fish,
            baby_fish,
            threshold: 0,
            spawning: 0,
        }
    }
}

impl Population {
    fn new(input: &str) -> Self {
        let mut population = Population::default();
        for fish in input.trim().split(",").map(|s| s.parse::<usize>().unwrap()) {
            population.adult_fish[fish] += 1;
        }
        population
    }

    fn update(&mut self) {
        let babies = self.adult_fish[self.spawning];
        self.adult_fish[self.spawning] += self.baby_fish[self.threshold];
        self.baby_fish[self.threshold] = babies;
        self.threshold = (self.threshold + 1) % self.baby_fish.len();
        self.spawning = (self.spawning + 1) % self.adult_fish.len();
    }

    fn simulate(&mut self, days: u16) -> u128 {
        for _ in 0..days {
            self.update();
        }
        self.baby_fish.iter().sum::<u128>() + self.adult_fish.iter().sum::<u128>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_simulate_18_days() {
        let mut population = Population::new(&INPUT);
        assert_eq!(population.simulate(18), 26);
    }

    #[test]
    fn test_simulate_80_days() {
        let mut population = Population::new(&INPUT);
        assert_eq!(population.simulate(80), 5934);
    }

    #[test]
    fn test_simulate_256_days() {
        let mut population = Population::new(&INPUT);
        assert_eq!(population.simulate(256), 26984457539);
    }
}
