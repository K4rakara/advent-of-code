//! # Day 2: Password Philosophy
//!
//! Your flight departs in a few days from the coastal airport; the easiest way
//! down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
//! "Something's wrong with our computers; we can't log in!"
//!
//! You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the
//! passwords wouldn't have been allowed by the Official Toboggan Corporate
//! Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input)
//! of passwords (according to the corrupted database) and the corporate policy
//! when that password was set.
//!
//! For example, suppose you have the following list:
//! 
//! ```ignore
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//! 
//! Each line gives the password policy and then the password. The password
//! policy indicates the lowest and highest number of times a given letter must
//! appear for the password to be valid. For example, `1-3` a means that the
//! password must contain a at least `1` time and at most `3` times.
//! 
//! In the above example, `2` passwords are valid. The middle password,
//! `cdefg`, is not; it contains no instances of `b`, but needs at least `1`.
//! The first and third passwords are valid: they contain one `a` or nine `c`,
//! both within the limits of their respective policies.
//! 
//! How many passwords are valid according to their policies?
//! 
//! My puzzle answer was `378`.
//! 
//! ### Part 2
//! 
//! While it appears you validated the passwords correctly, they don't seem to
//! be what the Official Toboggan Corporate Authentication System is expecting.
//! 
//! The shopkeeper suddenly realizes that he just accidentally explained the
//! password policy rules from his old job at the sled rental place down the
//! street! The Official Toboggan Corporate Policy actually works a little
//! differently.
//! 
//! Each policy actually describes two positions in the password, where 1 means
//! the first character, `2` means the second character, and so on. (Be
//! careful; Toboggan Corporate Policies have no concept of "index zero"!)
//! Exactly one of these positions must contain the given letter. Other
//! occurrences of the letter are irrelevant for the purposes of policy
//! enforcement.
//! 
//! Given the same example list from above:
//! 
//! ```ignore
//! 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//! 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//! 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//! ```
//!
//! How many passwords are valid according to the new interpretation of the policies?
//!
//! My puzzle answer was `280`.
//!

use crate::aoc;
use crate::lazy_static;
use crate::regex::Regex;

lazy_static! {
    static ref PASSWORD_LIMITATION_REGEX: Regex = Regex::new(r#"(\d+)-(\d+) ([a-zA-Z0-9])"#).unwrap();
}

pub type PasswordLimitation = ( char, usize, usize );

pub fn to_password_limitation(v: &str) -> PasswordLimitation {
    let captures = PASSWORD_LIMITATION_REGEX.captures(&v).unwrap();
    (
        captures[3].chars().next().unwrap(),
        captures[1].parse::<usize>().unwrap(),
        captures[2].parse::<usize>().unwrap(),
    )
}

#[aoc(day2, part1)]
pub fn part_1(v: &str) -> usize {
    v
        .split("\n")
        .map(|line| {
            let mut split = line.split(":");
            (
                to_password_limitation(split.next().unwrap()),
                split.next().unwrap().trim(),
            )
        })
        .filter(|(limitation, password)| {
            let count = password.matches(limitation.0).count();
            count >= limitation.1 && count <= limitation.2
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part_2(v: &str) -> usize {
    v
        .split("\n")
        .map(|line| {
            let mut split = line.split(":");
            (
                to_password_limitation(split.next().unwrap()),
                split.next().unwrap().trim(),
            )
        })
        .filter(|(limitation, password)| {
            let at_position_1 = password
                .chars()
                .skip(limitation.1 - 1)
                .next();
            let at_position_2 = password
                .chars()
                .skip(limitation.2 - 1)
                .next();
            (at_position_1 == Some(limitation.0))
            ^ (at_position_2 == Some(limitation.0))
        })
        .count()
}

