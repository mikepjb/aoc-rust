use std::fs;
use std::error::Error;

/// First attempt at day one:
///
/// This example fails because we equally subtract letters on the start and end of each input line
/// and that can remove the end portion of a written number match e.g nine before it gets matched
/// with starts_with

fn main() -> Result<(), Box<dyn Error>> {
    let day_input = fs::read_to_string("./input/day-one.txt")?;
    println!("{}", part_one(day_input));
    Ok(())
}

fn part_one(input: String) -> String {
    input.lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| calibration_values(line.to_string()))
        .sum::<i32>()
        .to_string()
}

const NUMERIC_VALUES: [(&str, i32); 18] =
[
    ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
    ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
];

/// Get the first/last digit on a line, these are the calibration values needed for the trebuchet..
/// to launch us into the sky correctly.
fn calibration_values(line: String) -> i32 {
    let (mut start_n, mut end_n) = (0, 0);
    let mut line = line;
    println!("{}", line);

    while (start_n == 0 || end_n == 0) && line.len() > 0 {
        for (s, n) in NUMERIC_VALUES.into_iter() {
            if line.starts_with(s) && start_n == 0 {
                start_n = n
            }

            if line.ends_with(s) && end_n == 0 {
                end_n = n
            }
        }

        // strip the first and last characters from line.
        let mut chars = line.chars();
        chars.next();
        if line.len() > 0 {
            chars.next_back();
        }
        line = chars.as_str().to_string();
    }

    println!("{}", start_n);
    if start_n == 0 {
        start_n = end_n
    } else if end_n == 0 {
        end_n = start_n
    };

    let out = (start_n.to_string() + &end_n.to_string()).to_string().parse::<i32>().unwrap();
    println!("{}:{}, {}", start_n, end_n, out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_single_line_part_one() {
        assert_eq!(calibration_values("1abc2".to_string()), 12);
        assert_eq!(calibration_values("2sdafs3".to_string()), 23);
        assert_eq!(calibration_values("2sdaf3s".to_string()), 23);
        assert_eq!(calibration_values("dfjnzxtlnine9five".to_string()), 95);
    }

    #[ignore]
    #[test]
    fn test_example_part_one() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part_one(example_input.to_string()), "142");
    }

    #[ignore]
    #[test]
    fn test_example_part_two() {
        let example_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part_one(example_input.to_string()), "281");
    }
}
