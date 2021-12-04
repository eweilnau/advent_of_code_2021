pub fn print_answers() {
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

fn sonar_sweep() -> Vec<i32> {
    std::fs::read_to_string("assets\\day_1_input.txt")
        .unwrap()
        .lines()
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
