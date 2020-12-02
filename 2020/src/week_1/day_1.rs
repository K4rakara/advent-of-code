use crate::aoc;

#[aoc(day1, part1)]
pub fn part_1(input_str: &str) -> i32 {
    let input = input_str
        .split("\n")
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    panic!("Solution doesn't exist???");
}

