use aoc_2024::read_input_to_string;

fn main() {
    let mut s = String::with_capacity(30_000);
    if let Ok(len) = read_input_to_string("input", &mut s) {
        let map = s
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| usize::try_from(c.to_digit(10).expect("error, c: {}")).expect("{c}"))
            .collect();
        let checksum = part_one(&map, len);
        println!("Checksum: {}", checksum);
    }
}

fn part_one(map: &Vec<usize>, len: usize) -> usize {
    let mut left = 0;
    let mut right = len - 1;

    // Determine whether the end is a file or free space
    // and set right pointer to the index of the first file
    // (every even index is a file)
    if (len - 1) % 2 != 0 {
        right -= 1;
    }

    // initialize by seeking to the first
    // free space, calculating checksum as we go
    // first checksum is 0 because file_id is 0
    let mut checksum = 0;
    let mut curr_block = map[left];
    left += 1;
    let mut blocks_free = map[left];
    let mut blocks_to_write = map[right];

    // while not compacted
    while left < right {
        // if there is nothing left to write
        if blocks_to_write == 0 {
            // if the next file has already been written / counted
            // we're done
            if right - 2 <= left {
                break;
            }

            // get the size of the next file
            right -= 2;
            blocks_to_write = map[right];
        }

        // if there is no space to write
        if blocks_free == 0 {
            // if we're currently writing the last file
            // finish writing it and exit
            if left == right - 1 {
                checksum += calc_checksum(blocks_to_write, curr_block, right / 2);
                break;
            }

            // seek through the next file_id until we reach the next free space
            // calculate the checksum as we go
            left += 1;
            // reuse blocks_free to hold the number of blocks to seek
            blocks_free = map[left];
            checksum += calc_checksum(blocks_free, curr_block, left / 2);
            curr_block += blocks_free;

            // get size of the next free space
            left += 1;
            blocks_free = map[left];
        }

        if blocks_to_write > blocks_free {
            // write enough file blocks to fill free space and calculate checksum
            blocks_to_write -= blocks_free;
            // file_id times sum of the integers from curr_block to (curr_block + num_block)
            checksum += calc_checksum(blocks_free, curr_block, right / 2);
            curr_block += blocks_free;
            blocks_free = 0;
        } else if blocks_free > blocks_to_write {
            // write the entire file to free space and calculate checksum
            blocks_free -= blocks_to_write;
            checksum += calc_checksum(blocks_to_write, curr_block, right / 2);
            curr_block += blocks_to_write;
            blocks_to_write = 0;
        } else {
            // blocks_free == blocks_to_write
            // write the entire file to free space and calclate checksum
            checksum += calc_checksum(blocks_to_write, curr_block, right / 2);
            curr_block += blocks_to_write;
            blocks_free = 0;
            blocks_to_write = 0;
        }
    }
    checksum
}

fn calc_checksum(num_blocks: usize, start: usize, id: usize) -> usize {
    if num_blocks == 0 {
        return 0;
    }
    id * ((num_blocks - 1) * (num_blocks + 2 * start) + 2 * start) / 2
}

#[cfg(test)]
mod tests {
    use crate::{calc_checksum, part_one};

    fn sum_series(start: usize, end: usize) -> usize {
        let num = end - start + 1;
        calc_checksum(num, start, 1)
    }

    #[test]
    fn test_sum_series() {
        assert_eq!(sum_series(1, 10), 55);
        assert_eq!(sum_series(1, 1), 1);
    }

    #[test]
    fn example_one() {
        let example = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        assert_eq!(part_one(&example, example.len()), 1928);
    }

    #[test]
    fn example_two() {
        let example = vec![1, 2, 3, 4, 5];
        assert_eq!(part_one(&example, example.len()), 60);
    }
}
