use aoc_2024::read_input_by_line;

enum Direction {
    Up,
    UpLeft,
    UpRight,
    Left,
    Right,
    Down,
    DownLeft,
    DownRight,
}

fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        let puzzle = lines.map(|l| l.unwrap().chars().collect()).collect();
        part_one(&puzzle);
        part_two(&puzzle);
    }
}

fn part_one(puzzle: &Vec<Vec<char>>) {
    let mut count = 0;

    for row in 0..puzzle.len() {
        for col in 0..puzzle[0].len() {
            if puzzle[row][col] == 'X' {
                start_search(row, col, puzzle, &mut count);
            }
        }
    }
    println!("Num XMAS appearances: {}", count);
}

fn start_search(row: usize, col: usize, puzzle: &Vec<Vec<char>>, count: &mut usize) {
    let mas = vec!['M', 'A', 'S'];
    if row >= 3 {
        if col >= 3 {
            search(row - 1, col - 1, puzzle, count, 0, &mas, Direction::UpLeft);
        }
        if col <= puzzle[0].len() - 4 {
            search(row - 1, col + 1, puzzle, count, 0, &mas, Direction::UpRight);
        }
        search(row - 1, col, puzzle, count, 0, &mas, Direction::Up);
    }
    if col >= 3 {
        search(row, col - 1, puzzle, count, 0, &mas, Direction::Left);
    }
    if col <= puzzle[0].len() - 4 {
        search(row, col + 1, puzzle, count, 0, &mas, Direction::Right);
    }
    if row <= puzzle.len() - 4 {
        if col >= 3 {
            search(
                row + 1,
                col - 1,
                puzzle,
                count,
                0,
                &mas,
                Direction::DownLeft,
            );
        }
        if col <= puzzle[0].len() - 4 {
            search(
                row + 1,
                col + 1,
                puzzle,
                count,
                0,
                &mas,
                Direction::DownRight,
            );
        }
        search(row + 1, col, puzzle, count, 0, &mas, Direction::Down);
    }
}

fn search(
    row: usize,
    col: usize,
    puzzle: &Vec<Vec<char>>,
    count: &mut usize,
    next: usize,
    mas: &Vec<char>,
    direction: Direction,
) -> bool {
    if puzzle[row][col] == mas[next] {
        // we've found XMAS
        if next >= mas.len() - 1 {
            *count += 1;
            return true;
        }
        let (nextrow, nextcol) = match direction {
            Direction::Up => (row - 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::UpLeft => (row - 1, col - 1),
            Direction::UpRight => (row - 1, col + 1),
            Direction::DownLeft => (row + 1, col - 1),
            Direction::DownRight => (row + 1, col + 1),
        };
        search(nextrow, nextcol, puzzle, count, next + 1, mas, direction)
    } else {
        // not a match
        return false;
    }
}

fn part_two(puzzle: &Vec<Vec<char>>) {
    let mut count = 0;

    for row in 1..puzzle.len() - 1 {
        for col in 1..puzzle[0].len() - 1 {
            // start with the center of the 'X'
            if puzzle[row][col] == 'A' {
                start_x_mas_search(row, col, puzzle, &mut count);
            }
        }
    }
    println!("Num X-MAS appearances: {}", count);
}

fn start_x_mas_search(row: usize, col: usize, puzzle: &Vec<Vec<char>>, count: &mut usize) {
    // check diagonals
    let (up_left, up_right, down_left, down_right) = (
        puzzle[row - 1][col - 1],
        puzzle[row - 1][col + 1],
        puzzle[row + 1][col - 1],
        puzzle[row + 1][col + 1],
    );
    // check top left to bottom right diagonal (\) forwards and backwards
    if up_left == 'M' && down_right == 'S' || up_left == 'S' && down_right == 'M' {
        // check bottom left to top right diagonal (/) forwards and backwards
        if down_left == 'M' && up_right == 'S' || down_left == 'S' && up_right == 'M' {
            *count += 1;
        }
    }
}
