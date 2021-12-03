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
    let (position, depth, _) = course.iter().fold((0, 0, 0), | (position, depth, aim), course_step | match course_step {
        CourseStep::Forward(value) => (position + value, depth + aim * value, aim),
        CourseStep::Down(value) => (position, depth, aim + value),
        CourseStep::Up(value) => (position, depth, aim - value),
    });
    (position, depth)
}
