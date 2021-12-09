pub fn print_answers() {
    println!("\n--- Day 9: Smoke Basin ---");
    let map = HeightMap::new(&std::fs::read_to_string("assets\\day_09_input.txt").unwrap());
    println!(
        "What is the sum of the risk levels of all low points on your heightmap? {}",
        map.get_risk_level()
    );
    let mut basins = map.get_basins();
    basins.sort_by_key(|b| b.len());
    let value = basins
        .iter()
        .rev()
        .take(3)
        .map(|b| b.len())
        .product::<usize>();
    println!(
        "What do you get if you multiply together the sizes of the three largest basins? {}",
        value
    );
}

struct HeightMap {
    data: Vec<u32>,
    columns: usize,
}

impl HeightMap {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let columns = lines[0].chars().count();
        let data: Vec<u32> = lines
            .iter()
            .flat_map(|&line| line.chars().map(|c| c.to_digit(10).unwrap() as u32))
            .collect();
        Self { data, columns }
    }

    fn get_neighbors(&self, index: usize) -> Vec<(usize, u32)> {
        let mut neighbor_index = Vec::new();
        if index >= self.columns {
            neighbor_index.push(index - self.columns)
        }
        if index % self.columns != 0 {
            neighbor_index.push(index - 1)
        }
        if index % self.columns != self.columns - 1 {
            neighbor_index.push(index + 1)
        }
        if index < self.data.len() - self.columns {
            neighbor_index.push(index + self.columns)
        }
        neighbor_index
            .iter()
            .map(move |&i| (i, self.data[i]))
            .collect()
    }

    fn get_low_points(&self) -> Vec<(usize, u32)> {
        self.data
            .iter()
            .enumerate()
            .filter(|(i, v)| self.get_neighbors(*i).iter().all(|(_, nv)| *v < nv))
            .map(|(i, &v)| (i, v))
            .collect()
    }

    fn get_risk_level(&self) -> u32 {
        self.get_low_points().iter().map(|(_, v)| *v + 1).sum()
    }

    fn get_basins(&self) -> Vec<std::collections::HashSet<(usize, u32)>> {
        let mut basins = Vec::new();
        for low_point in self.get_low_points() {
            let mut queue = std::collections::HashSet::new();
            let mut basin = std::collections::HashSet::new();
            queue.insert(low_point);

            while queue.len() > 0 {
                let tile = queue.iter().copied().next().unwrap();
                queue.remove(&tile);
                if !basin.contains(&tile) {
                    basin.insert(tile);
                }
                for neighbor in self.get_neighbors(tile.0) {
                    if neighbor.1 < 9 && !basin.contains(&neighbor) {
                        queue.insert(neighbor);
                    }
                }
            }
            basins.push(basin);
        }
        basins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_get_risk_level() {
        let map = HeightMap::new(INPUT);
        assert_eq!(map.get_risk_level(), 15);
    }

    #[test]
    fn test_get_basins() {
        let map = HeightMap::new(INPUT);
        let mut basins = map.get_basins();
        basins.sort_by_key(|b| b.len());
        let value = basins
            .iter()
            .rev()
            .take(3)
            .map(|b| b.len())
            .product::<usize>();
        assert_eq!(value, 1134);
    }

    #[test]
    fn test_get_low_points() {
        let map = HeightMap::new(INPUT);
        let expected = vec![(1, 1), (9, 0), (22, 5), (46, 5)];
        assert_eq!(map.get_low_points(), expected);
    }

    #[test]
    fn test_get_neighbors() {
        let map = HeightMap::new(INPUT);
        let expected = vec![(3, 9), (12, 8), (14, 8), (23, 6)];
        assert_eq!(map.get_neighbors(13), expected);
    }

    #[test]
    fn test_get_neighbors_top_edge() {
        let map = HeightMap::new(INPUT);
        let expected = vec![(4, 9), (6, 3), (15, 9)];
        assert_eq!(map.get_neighbors(5), expected);
    }

    #[test]
    fn test_get_neighbors_bottom_edge() {
        let map = HeightMap::new(INPUT);
        let expected = vec![(36, 6), (45, 6), (47, 6)];
        assert_eq!(map.get_neighbors(46), expected);
    }

    #[test]
    fn test_get_neighbors_left_edge() {
        let map = HeightMap::new(INPUT);
        let expected = vec![(10, 3), (21, 8), (30, 8)];
        assert_eq!(map.get_neighbors(20), expected);
    }

    #[test]
    fn test_get_neighbors_right_edge() {
        let map = HeightMap::new(INPUT);
        let expected = vec![(9, 0), (18, 2), (29, 2)];
        assert_eq!(map.get_neighbors(19), expected);
    }

    #[test]
    fn test_get_neighbors_corner() {
        let map = HeightMap::new(INPUT);
        let expected = vec![(30, 8), (41, 8)];
        assert_eq!(map.get_neighbors(40), expected);
    }
}
