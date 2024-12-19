use std::collections::{HashMap, HashSet};

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
        calculate_scores(&map);
    }
}

fn calculate_scores(map: &Vec<Vec<u32>>) {
    let (mut part_one, mut part_two) = (0, 0);
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 9 {
                // search for trailheads reachable from the top
                // keeping track of which were reachable
                let mut visited = HashMap::new();
                search(map, row, col, &mut visited);
                part_one += visited.len();
                part_two += visited.into_values().sum::<usize>();
            }
        }
    }
    println!("Sum of trailhead scores: {}", part_one);
    println!("Sum of trailhead ratings: {}", part_two);
}

fn search(
    map: &Vec<Vec<u32>>,
    row: usize,
    col: usize,
    ratings: &mut HashMap<(usize, usize), usize>,
) {
    let height = map[row][col];

    // if we've reached a trailhead
    if height == 0 {
        // add it or increment its rating by one
        ratings
            .entry((row, col))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    } else if height != 0 {
        // search up
        if row > 0 && map[row - 1][col] == height - 1 {
            search(map, row - 1, col, ratings);
        }

        // search left
        if col > 0 && map[row][col - 1] == height - 1 {
            search(map, row, col - 1, ratings);
        }

        // search right
        if col < map[0].len() - 1 && map[row][col + 1] == height - 1 {
            search(map, row, col + 1, ratings);
        }

        // search down
        if row < map.len() - 1 && map[row + 1][col] == height - 1 {
            search(map, row + 1, col, ratings);
        }
    }
}
