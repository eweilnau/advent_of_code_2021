pub fn print_answers() {
    println!("\n--- Day 8: Seven Segment Search ---");
    let count = std::fs::read_to_string("assets\\day_08_input.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(" | ").nth(1).unwrap().split_whitespace())
        .flatten()
        .filter(|&s| [2usize, 3usize, 4usize, 7usize].contains(&s.len()))
        .count();
    println!(
        "In the output values, how many times do digits 1, 4, 7, or 8 appear? {}",
        count
    );
    let display_total: u64 = std::fs::read_to_string("assets\\day_08_input.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let display = Display::new(s);
            display.get_output_value()
        })
        .sum();
    println!(
        "What do you get if you add up all of the output values? {}",
        display_total
    );
}

struct Display {
    data: String,
}

impl Display {
    fn new(input: &str) -> Self {
        let data = String::from(input);
        Self { data }
    }

    fn get_output_value(&self) -> u64 {
        let mut parts = self.data.split(" | ");
        let wires: Vec<char> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars())
            .flatten()
            .collect();
        let digits: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| match wires.iter().filter(|&&w| s.contains(w)).count() {
                42 => 0,
                17 => 1,
                34 => 2,
                39 => 3,
                30 => 4,
                37 => 5,
                41 => 6,
                25 => 7,
                49 => 8,
                45 => 9,
                _ => panic!("unknown digit: {}", s),
            })
            .collect();
        digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test() {
        let count = INPUT
            .lines()
            .map(|line| line.split(" | ").nth(1).unwrap().split_whitespace())
            .flatten()
            .filter(|&s| [2usize, 3usize, 4usize, 7usize].contains(&s.len()))
            .count();
        assert_eq!(count, 26);
    }

    #[test]
    fn test_display_total() {
        let display_total: u64 = INPUT
            .lines()
            .map(|s| {
                let display = Display::new(s);
                display.get_output_value()
            })
            .sum();
        assert_eq!(display_total, 61229)
    }

    #[test]
    fn test_display_output_value() {
        let display = Display::new(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(display.get_output_value(), 5353)
    }
}
