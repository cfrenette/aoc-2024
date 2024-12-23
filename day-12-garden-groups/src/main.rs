use aoc_2024::read_input_by_line;
use std::collections::VecDeque;

fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        let garden = lines.map(|l| l.unwrap().chars().collect()).collect();
        part_one(&garden);
    }
}

fn part_one(garden: &Vec<Vec<char>>) {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut total = 0;
    for row in 0..garden.len() {
        for col in 0..garden[row].len() {
            if !visited[row][col] {
                let (a, p) = flood_fill(&mut visited, garden, row, col);
                total += a * p;
            }
        }
    }
    println!("Total price: {}", total);
}

fn flood_fill(
    visited: &mut Vec<Vec<bool>>,
    garden: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> (usize, usize) {
    let group = garden[row][col];
    let mut to_visit = VecDeque::new();
    let mut perim = 0;
    let mut area = 0;

    to_visit.push_front((row, col));

    while let Some((r, c)) = to_visit.pop_back() {
        if visited[r][c] {
            continue;
        }
        area += 1;
        visited[r][c] = true;
        if r > 0 && garden[r - 1][c] == group {
            if !visited[r - 1][c] {
                to_visit.push_front((r - 1, c));
            }
        } else {
            perim += 1;
        }
        if c > 0 && garden[r][c - 1] == group {
            if !visited[r][c - 1] {
                to_visit.push_front((r, c - 1));
            }
        } else {
            perim += 1;
        }
        if r < garden.len() - 1 && garden[r + 1][c] == group {
            if !visited[r + 1][c] {
                to_visit.push_front((r + 1, c));
            }
        } else {
            perim += 1;
        }
        if c < garden[r].len() - 1 && garden[r][c + 1] == group {
            if !visited[r][c + 1] {
                to_visit.push_front((r, c + 1));
            }
        } else {
            perim += 1;
        }
    }

    (area, perim)
}
