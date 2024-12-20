use std::collections::HashMap;

use aoc_2024::read_input_to_string;

fn main() {
    let mut buf = String::with_capacity(100);
    if let Ok(_len) = read_input_to_string("input", &mut buf) {
        part_one(&buf);
        part_two(&buf);
    }
}

fn part_one(start_str: &String) {
    let mut total = 0;
    for stone in start_str.split_whitespace() {
        blink(stone.parse().unwrap(), 25, &mut total);
    }
    println!("Number of stones: {}", total);
}

fn part_two(start_str: &String) {
    let mut memo = HashMap::new();
    let mut total = 0;
    for stone in start_str.split_whitespace() {
        let subtotal = smart_blink(stone.parse().unwrap(), 75, &mut memo);
        memo.insert((stone.parse().unwrap(), 75), subtotal);
        total += subtotal;
    }
    println!("Number of stones, part two: {}", total);
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

fn smart_blink(stone: usize, blinks: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(num) = memo.get(&(stone, blinks)) {
        return *num;
    }
    if blinks == 0 {
        return 1;
    }
    if stone == 0 {
        let total = smart_blink(1, blinks - 1, memo);
        memo.insert((stone, blinks), total);
        return total;
    }
    let digits = stone.to_string();
    if digits.len() % 2 == 0 {
        let (left, right) = digits.split_at(digits.len() / 2);
        let left_total = smart_blink(left.parse().unwrap(), blinks - 1, memo);
        let right_total = smart_blink(right.parse().unwrap(), blinks - 1, memo);
        memo.insert((stone, blinks), left_total + right_total);
        return left_total + right_total;
    }
    let total = smart_blink(stone * 2024, blinks - 1, memo);
    memo.insert((stone, blinks), total);
    return total;
}
