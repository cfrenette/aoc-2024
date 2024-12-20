use aoc_2024::read_input_to_string;

fn main() {
    let mut buf = String::with_capacity(100);
    if let Ok(_len) = read_input_to_string("input", &mut buf) {
        part_one(&buf);
    }
}

fn part_one(start_str: &String) {
    let mut total = 0;
    for stone in start_str.split_whitespace() {
        blink(stone.parse().unwrap(), 25, &mut total);
    }
    println!("Number of stones: {}", total);
}

fn blink(stone: usize, blinks: usize, total: &mut usize) {
    if blinks == 0 {
        *total += 1;
        return;
    }
    if stone == 0 {
        blink(1, blinks - 1, total);
        return;
    }
    let digits = stone.to_string();
    if digits.len() % 2 == 0 {
        let (left, right) = digits.split_at(digits.len() / 2);
        blink(left.parse().unwrap(), blinks - 1, total);
        blink(right.parse().unwrap(), blinks - 1, total);
        return;
    }
    blink(stone * 2024, blinks - 1, total);
    return;
}
