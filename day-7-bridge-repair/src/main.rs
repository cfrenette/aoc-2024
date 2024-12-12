use std::{
    fs::File,
    io::{BufReader, Lines},
};

use aoc_2024::read_input_by_line;

fn main() {
    if let Ok(mut lines) = read_input_by_line("input") {
        part_one(&mut lines)
    }
}

fn part_one(lines: &mut Lines<BufReader<File>>) {
    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let nums = line
            .split_whitespace()
            .map(|n| n.replace(':', "").parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        // try every possible combination (TODO: is it possible to do better?)
        if dp_recurse(nums[0], 0, 1, &nums) {
            sum += nums[0];
        }
    }
    println!("sum of possible values: {}", sum);
}

fn dp_recurse(target: usize, sum: usize, i: usize, nums: &[usize]) -> bool {
    if i == nums.len() {
        return target == sum;
    }
    dp_recurse(target, sum * nums[i], i + 1, nums) || dp_recurse(target, sum + nums[i], i + 1, nums)
}
