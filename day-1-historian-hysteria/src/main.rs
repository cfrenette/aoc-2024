use aoc_2024::read_input_by_line;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_input_by_line("input") {
        let lists = lines.map(|line_result| {
            let line = line_result.unwrap();
            let mut list_elements = line.split_whitespace();
            // If either are None or not unsigned integers we should
            // panic because an assumption has been violated
            (
                list_elements.next().unwrap().parse::<usize>().unwrap(),
                list_elements.next().unwrap().parse::<usize>().unwrap(),
            )
        });
        let (mut left_list, mut right_list) = lists.unzip();
        part_one(&mut left_list, &mut right_list);
        part_two(&left_list, &right_list);
    }
}

fn part_one(left_list: &mut Vec<usize>, right_list: &mut Vec<usize>) {
    left_list.sort();
    right_list.sort();
    let accum = |acc, e| {
        let (left, right) = e;
        if left <= right {
            acc + (right - left)
        } else {
            acc + (left - right)
        }
    };
    let zipped = left_list.iter().zip(right_list.iter());
    println!("Total distance: {}", zipped.fold(0, accum));
}

fn part_two(left_list: &Vec<usize>, right_list: &Vec<usize>) {
    // populate multiplicities
    let mut multiplicity_map = HashMap::new();
    for e in right_list {
        let multiplicity = match multiplicity_map.get(&e) {
            Some(m) => *m,
            None => 0,
        };
        multiplicity_map.insert(e, multiplicity + 1);
    }

    // calculate similarity score
    let result = left_list.iter().fold(0, |acc, e| {
        acc + (e * multiplicity_map.get(&e).unwrap_or_else(|| &0))
    });
    println!("Similarity score: {}", result);
}
