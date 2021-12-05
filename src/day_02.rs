pub fn print_answers() {
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

enum CourseStep {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn get_course() -> Vec<CourseStep> {
    std::fs::read_to_string("assets\\day_2_input.txt")
        .unwrap()
        .lines()
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
