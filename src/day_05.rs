pub fn print_answers() {
    println!("\n--- Day 5: Hydrothermal Venture ---");
    let input = std::fs::read_to_string("assets\\day_05_input.txt").unwrap();
    let map = VentMap::new(&input, false);
    println!(
        "Considering only horizontal and vertical vent lines, how many points do at least two vent lines overlap? {}",
        map.get_multiple_vent_point_count()
    );
    let map = VentMap::new(&input, true);
    println!(
        "Considering all vent lines, how many points do at least two vent lines overlap? {}",
        map.get_multiple_vent_point_count()
    );
}

#[derive(Default)]
struct VentMap {
    vents: std::collections::HashMap<Point, u16>,
}

impl VentMap {
    fn new(input: &str, include_diagonal: bool) -> Self {
        let mut map = VentMap::default();

        for line in input.lines() {
            map.plot_line(line, include_diagonal);
        }

        map
    }

    fn plot_line(&mut self, line: &str, include_diagonal: bool) {
        let mut points: Vec<Point> = line
            .split_whitespace()
            .filter(|&s| s.contains(","))
            .map(|s| s.parse::<Point>().unwrap())
            .collect();
        let start = points.remove(0);
        let end = points.remove(0);

        if start.y == end.y {
            for point in horizontal_line(start, end) {
                let vent = self.vents.entry(point).or_insert(0);
                *vent += 1;
            }
        }

        if start.x == end.x {
            for point in vertical_line(start, end) {
                let vent = self.vents.entry(point).or_insert(0);
                *vent += 1;
            }
        }

        if start.x != end.x && start.y != end.y && include_diagonal {
            for point in diagonal_line(start, end) {
                let vent = self.vents.entry(point).or_insert(0);
                *vent += 1;
                drop(vent);
            }
        }
    }

    fn get_multiple_vent_point_count(&self) -> usize {
        self.vents.values().filter(|&&count| count > 1).count()
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(",").collect();
        let x = coords[0].parse::<u16>()?;
        let y = coords[1].parse::<u16>()?;
        Ok(Self { x, y })
    }
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

fn horizontal_line(start: Point, end: Point) -> impl Iterator<Item = Point> {
    assert_eq!(start.y, end.y);
    let x1 = std::cmp::min(start.x, end.x);
    let x2 = std::cmp::max(start.x, end.x) + 1;
    let y = start.y;
    (x1..x2).map(move |x| Point::new(x, y))
}

fn vertical_line(start: Point, end: Point) -> impl Iterator<Item = Point> {
    assert_eq!(start.x, end.x);
    let y1 = std::cmp::min(start.y, end.y);
    let y2 = std::cmp::max(start.y, end.y) + 1;
    let x = start.x;
    (y1..y2).map(move |y| Point::new(x, y))
}

fn diagonal_line(start: Point, end: Point) -> impl Iterator<Item = Point> {
    let len = std::cmp::max(start.x, end.x) - std::cmp::min(start.x, end.x) + 1;
    let mut x = start.x;
    let mut y = start.y;
    (0..len).map(move |i| {
        let point = Point::new(x, y);
        if i < len - 1 {
            if start.x > end.x {
                x -= 1;
            } else {
                x += 1;
            }
            if start.y > end.y {
                y -= 1;
            } else {
                y += 1;
            }
        }
        point
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_get_multiple_vent_point_count() {
        let map = VentMap::new(INPUT, false);
        assert_eq!(map.get_multiple_vent_point_count(), 5);
    }

    #[test]
    fn test_get_multiple_vent_point_count_with_diagonal() {
        let map = VentMap::new(INPUT, true);
        assert_eq!(map.get_multiple_vent_point_count(), 12);
    }

    #[test]
    fn test_horizontal_vent_line() {
        let map = VentMap::new("6,0 -> 3,0", false);
        assert!((3..7)
            .map(|x| Point::new(x, 0))
            .all(|point| map.vents.contains_key(&point)));
    }

    #[test]
    fn test_vertical_vent_line() {
        let map = VentMap::new("1,3 -> 1,5", false);
        assert!((3..6)
            .map(|y| Point::new(1, y))
            .all(|point| map.vents.contains_key(&point)));
    }

    #[test]
    fn test_diagonal_vent_line() {
        let map = VentMap::new("0,2 -> 2,0", true);
        assert!([Point::new(0, 2), Point::new(1, 1), Point::new(2, 0)]
            .iter()
            .all(|point| map.vents.contains_key(&point)));
    }

    #[test]
    fn test_overlapping_vent_lines_increases_vent_count() {
        let map = VentMap::new(INPUT, false);
        assert_eq!(map.vents[&Point::new(0, 9)], 2);
    }
}
