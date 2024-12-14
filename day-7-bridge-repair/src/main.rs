use std::{
    fs::File,
    io::{BufReader, Lines},
};

use aoc_2024::read_input_by_line;

fn main() {
    if let Ok(mut lines) = read_input_by_line("input") {
        part_one(&mut lines)
    }
    if let Ok(mut lines) = read_input_by_line("input") {
        part_two(&mut lines)
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
    println!("sum of possible values, part one: {}", sum);
}

fn part_two(lines: &mut Lines<BufReader<File>>) {
    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let nums = line.split_whitespace().collect::<Vec<&str>>();
        let target = nums[0].replace(':', "").parse().unwrap();
        if dp_recurse_str(target, 0, 1, &nums) {
            sum += target;
        }
    }
    println!("sum of possible values, part two: {}", sum);
}

fn dp_recurse(target: usize, sum: usize, i: usize, nums: &[usize]) -> bool {
    if i == nums.len() {
        return target == sum;
    }
    dp_recurse(target, sum * nums[i], i + 1, nums) || dp_recurse(target, sum + nums[i], i + 1, nums)
}

fn dp_recurse_str(target: usize, sum: usize, i: usize, nums: &[&str]) -> bool {
    if i == nums.len() {
        return target == sum;
    }
    let this_num = nums[i].parse::<usize>().unwrap();
    let concat = sum.to_string() + nums[i];
    dp_recurse_str(target, sum * this_num, i + 1, nums)
        || dp_recurse_str(target, sum + this_num, i + 1, nums)
        || dp_recurse_str(target, concat.parse().unwrap(), i + 1, nums)
}

#[cfg(test)]
mod tests {
    use crate::dp_recurse_str;

    fn test_example(s: &str) -> bool {
        let nums = s.split_whitespace().collect::<Vec<&str>>();
        let target = nums[0].replace(':', "").parse().unwrap();
        dp_recurse_str(target, 0, 1, &nums)
    }
    #[test]
    fn test_examples() {
        let line_one = "156: 15 6";
        let line_two = "7290: 6 8 6 15";
        let line_three = "192: 17 8 14";

        assert!(test_example(line_one));
        assert!(test_example(line_two));
        assert!(test_example(line_three));
    }
}
