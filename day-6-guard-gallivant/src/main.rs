use aoc_2024::read_input_by_line;

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn turn(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

fn main() {
    if let Ok(mut lines) = read_input_by_line("input") {
        let mut map = Vec::with_capacity(lines.size_hint().0);
        let mut pos = (0, 0);
        let mut dir = Direction::Up;

        // Search for the guard while loading the map
        let mut guard_found = false;
        let mut row = 0;
        while let Some(Ok(line)) = lines.next() {
            let line: Vec<char> = line.chars().collect();
            if !guard_found {
                let mut col = 0;
                for chr in &line {
                    match *chr {
                        '^' => {
                            (pos, dir) = ((row, col), Direction::Up);
                            guard_found = true;
                        }
                        '<' => {
                            (pos, dir) = ((row, col), Direction::Left);
                            guard_found = true;
                        }
                        '>' => {
                            (pos, dir) = ((row, col), Direction::Right);
                            guard_found = true;
                        }
                        'v' => {
                            (pos, dir) = ((row, col), Direction::Down);
                            guard_found = true;
                        }
                        _ => {}
                    }
                    col += 1;
                }
                row += 1;
            }
            map.push(line)
        }

        part_one(&mut map, pos, dir);
    }
}

fn part_one(map: &mut Vec<Vec<char>>, pos: (usize, usize), dir: Direction) {
    let mut count = 0;
    let (mut row, mut col) = pos;
    let mut dir = dir;

    // TODO: handle a cycle in-bounds?
    loop {
        // mark visited and count
        if map[row][col] != 'X' {
            count += 1;
            map[row][col] = 'X';
        }

        // determine the next position and direction
        if let Some(next) = next_in_bounds(map, &mut dir, row, col) {
            // if the next position is in bounds, move
            (row, col) = next;
        } else {
            // otherwise guard leaves the map
            break;
        }
    }

    println!("Number of distinct positions visited: {}", count);
}

fn next_in_bounds(
    map: &Vec<Vec<char>>,
    dir: &mut Direction,
    row: usize,
    col: usize,
) -> Option<(usize, usize)> {
    let max_row = map.len() - 1;
    let max_col = map[row].len() - 1;

    let next_pos = |dir: &mut Direction, row, col| match *dir {
        Direction::Up => {
            if row > 0 {
                Some((row - 1, col))
            } else {
                None
            }
        }
        Direction::Left => {
            if col > 0 {
                Some((row, col - 1))
            } else {
                None
            }
        }
        Direction::Right => {
            if col < max_col {
                Some((row, col + 1))
            } else {
                None
            }
        }
        Direction::Down => {
            if row < max_row {
                Some((row + 1, col))
            } else {
                None
            }
        }
    };

    // while the next position is in bounds
    while let Some(next) = next_pos(dir, row, col) {
        // if the next position isn't obstructed
        if map[next.0][next.1] != '#' {
            // return it
            return Some(next);
        } else {
            // otherwise change direction
            dir.turn();
        }
    }
    // position out of bounds, guard leaves the map
    None
}
