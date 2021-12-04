use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Advent of Code 2021");

    {
        println!("\n--- Day 1: Sonar Sweep ---");
        let depth_measurements = sonar_sweep();
        println!(
            "How many measurements are larger than the previous measurement? {}",
            count_depth_measurement_increases(&depth_measurements)
        );
        println!(
            "Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum? {}",
            count_depth_sliding_sum_increases(&depth_measurements, 3)
        );
    }

    {
        println!("\n--- Day 2: Dive! ---");
        let course = get_course();
        let (position, depth) = plot_course_incorrect(&course);
        println!(
            "What do you get if you multiply your final horizontal position by your final depth? {}",
            position * depth
        );
        let (position, depth) = plot_course(&course);
        println!(
            "Using the new interpretation of the commands, what do you get if you multiply your final horizontal position by your final depth? {}",
            position * depth
        )
    }

    {
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
}

const ASSET_PATH: &str = "C:\\Users\\eweilnau\\projects\\rust\\advent_of_code_2021\\assets\\";

fn get_input(day: i32) -> impl Iterator<Item = String> {
    let path = format!("{}day_{}_input.txt", ASSET_PATH, day);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap())
}

fn sonar_sweep() -> Vec<i32> {
    get_input(1)
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn count_depth_measurement_increases(data: &[i32]) -> i32 {
    let mut results = 0;
    for i in 1..data.len() {
        if data[i - 1] < data[i] {
            results += 1;
        }
    }
    results
}

fn count_depth_sliding_sum_increases(data: &[i32], sliding_window_size: usize) -> i32 {
    let mut results = 0;
    for i in 1..data.len() - (sliding_window_size - 1) {
        let previous_sum = data[i - 1..i + (sliding_window_size - 1)]
            .iter()
            .sum::<i32>();
        let current_sum = data[i..i + sliding_window_size].iter().sum::<i32>();
        if previous_sum < current_sum {
            results += 1
        }
    }
    results
}

enum CourseStep {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn get_course() -> Vec<CourseStep> {
    get_input(2)
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let value = parts[1].parse::<i32>().unwrap();
            match parts[0] {
                "forward" => CourseStep::Forward(value),
                "down" => CourseStep::Down(value),
                "up" => CourseStep::Up(value),
                _ => panic!("Invalid Format: {}", line),
            }
        })
        .collect()
}

fn plot_course_incorrect(course: &[CourseStep]) -> (i32, i32) {
    course
        .iter()
        .fold((0, 0), |(position, depth), course_step| match course_step {
            CourseStep::Forward(value) => (position + value, depth),
            CourseStep::Down(value) => (position, depth + value),
            CourseStep::Up(value) => (position, depth - value),
        })
}

fn plot_course(course: &[CourseStep]) -> (i32, i32) {
    let (position, depth, _) = course.iter().fold(
        (0, 0, 0),
        |(position, depth, aim), course_step| match course_step {
            CourseStep::Forward(value) => (position + value, depth + aim * value, aim),
            CourseStep::Down(value) => (position, depth, aim + value),
            CourseStep::Up(value) => (position, depth, aim - value),
        },
    );
    (position, depth)
}

fn get_diagnostic_report() -> Vec<u16> {
    get_input(3)
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
    let oxygen_generator_rating: u64 =
        filter(
            report.clone(),
            |count, length| {
                if count as f32 >= length as f32 / 2.0 {
                    1
                } else {
                    0
                }
            },
        )
        .into();
    let co2_scrubber_rating: u64 = filter(
        report.clone(),
        |count, length| {
            if (count as f32) < length as f32 / 2.0 {
                1
            } else {
                0
            }
        },
    )
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
