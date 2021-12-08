pub fn print_answers() {
    println!("\n--- Day 3: Binary Diagnostic ---");
    let report = get_diagnostic_report();
    println!(
        "What is the power consumption of the submarine? {}",
        calculate_power_consumption(&report)
    );
    println!(
        "What is the life support rating of the submarine? {}",
        calculate_life_support_rating(report)
    )
}

fn get_diagnostic_report() -> Vec<u16> {
    std::fs::read_to_string("assets\\day_03_input.txt")
        .unwrap()
        .lines()
        .map(|line| u16::from_str_radix(&line, 2).unwrap())
        .collect()
}

fn calculate_power_consumption(report: &[u16]) -> u64 {
    let bit_count = report.iter().fold(vec![0; 12], |mut acc, entry| {
        for i in 0..12 {
            acc[i] += (entry >> i) & 1;
        }
        acc
    });
    let gamma_rate = u64::from_str_radix(
        &bit_count
            .iter()
            .rev()
            .map(|count| {
                if *count > report.len() as u16 / 2 {
                    "1"
                } else {
                    "0"
                }
            })
            .collect::<String>(),
        2,
    )
    .unwrap();
    let epsilon_rate = !gamma_rate & 0b111111111111;
    gamma_rate * epsilon_rate
}

fn calculate_life_support_rating(report: Vec<u16>) -> u64 {
    let oxygen_generator_rating: u64 = filter(report.clone(), |count, length| {
        if count as f32 >= length as f32 / 2.0 {
            1
        } else {
            0
        }
    })
    .into();
    let co2_scrubber_rating: u64 = filter(report.clone(), |count, length| {
        if (count as f32) < length as f32 / 2.0 {
            1
        } else {
            0
        }
    })
    .into();
    oxygen_generator_rating * co2_scrubber_rating
}

fn filter<F>(mut report: Vec<u16>, bit_criteria: F) -> u16
where
    F: Fn(usize, usize) -> u16,
{
    let mut bit_index = 11;
    while report.len() > 1 {
        let bit_count = report
            .iter()
            .fold(0, |acc, entry| acc + ((entry >> bit_index) & 1));
        let mask = bit_criteria(bit_count as usize, report.len());
        report = report
            .iter()
            .filter(|&&entry| (entry >> bit_index) & 1 == mask)
            .cloned()
            .collect();
        bit_index -= 1;
    }
    report[0]
}
