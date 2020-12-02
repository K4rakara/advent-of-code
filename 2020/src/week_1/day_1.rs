use crate::aoc; 

pub fn str_to_vec_i32(s: &str) -> Vec<i32> {
    s
        .split("\n")
        // The input is from a trusted source, don't yell at me for using
        // unwrap in this case, ffs.
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[aoc(day1, part1)]
pub fn part_1(input_str: &str) -> i32 {
    let input = str_to_vec_i32(input_str);
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    panic!("Solution doesn't exist???");
}

#[aoc(day1, part2)]
pub fn part_2(input_str: &str) -> i32 {
    let input = str_to_vec_i32(input_str);
    for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    panic!("Solution doesn't exist???");
}

