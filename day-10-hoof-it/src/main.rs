use std::collections::HashSet;

use aoc_2024::read_input_by_line;

fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        let map = lines
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        part_one(&map);
    }
}

fn part_one(map: &Vec<Vec<u32>>) {
    let mut sum = 0;
    for row in 0..map.len() {
        for col in 0..map.len() {
            if map[row][col] == 9 {
                // search for trailheads reachable from the top
                // keeping track of which were reachable
                let mut visited = HashSet::new();
                search(map, row, col, &mut visited);
                sum += visited.len();
            }
        }
    }
    println!("Sum of trailhead scores: {}", sum);
}

fn search(map: &Vec<Vec<u32>>, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>) {
    let height = map[row][col];

    // if we've reached a new trailhead, add it to the list
    if height == 0 && !visited.contains(&(row, col)) {
        // mark the trailhead as visited so we don't double count it
        visited.insert((row, col));
    } else if height != 0 {
        // search up
        if row > 0 && map[row - 1][col] == height - 1 {
            search(map, row - 1, col, visited);
        }

        // search left
        if col > 0 && map[row][col - 1] == height - 1 {
            search(map, row, col - 1, visited);
        }

        // search right
        if col < map[0].len() - 1 && map[row][col + 1] == height - 1 {
            search(map, row, col + 1, visited);
        }

        // search down
        if row < map.len() - 1 && map[row + 1][col] == height - 1 {
            search(map, row + 1, col, visited);
        }
    }
}
