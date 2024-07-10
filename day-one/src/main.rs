use std::fs;
use std::error::Error;

const NUMERICAL_VALUES: [(&str, &str); 9] = [
    ("1", "1"), ("2", "2"), ("3", "3"), ("4", "4"), ("5", "5"),
    ("6", "6"), ("7", "7"), ("8", "8"), ("9", "9")
];

const TEXT_VALUES: [(&str, &str); 9] = [
    ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"),
    ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")
];

fn main() -> Result<(), Box<dyn Error>> {
    let day_input = fs::read_to_string("./input/day-one.txt")?;

    let mut all_values = Vec::new();
    all_values.extend_from_slice(&TEXT_VALUES);
    all_values.extend_from_slice(&NUMERICAL_VALUES);

    println!("{}", part_one(day_input.clone()));
    println!("{}", part_two(day_input.clone()));
    Ok(())
}

fn part_one(input: String) -> i32 {
    let mut values = Vec::new();
    values.extend_from_slice(&NUMERICAL_VALUES);
    sum_lines(input, values)
}

fn part_two(input: String) -> i32 {
    let mut values = Vec::new();
    values.extend_from_slice(&NUMERICAL_VALUES);
    values.extend_from_slice(&TEXT_VALUES);
    sum_lines(input, values)
}

fn sum_lines(input: String, values: Vec<(&str, &str)>) -> i32 {
    input.lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| calibration_values(line.to_string(), &values))
        .sum::<i32>()
}

fn calibration_values(line: String, values: &Vec<(&str, &str)>) -> i32 {
    // find index for all values.. pick the largest and smallest indexes and use their values
    let mut maxt: (i32, &str) = (-1, "");
    let mut mint: (i32, &str) = (-1, "");

    for (m, v) in values {
        let (max, min) = (maxt.0, mint.0);
        if let Some(i) = line.find(m) {
            let index = i.try_into().unwrap(); // from unsigned to signed
            if index < min || min == -1 {
                mint = (index, v)
            }

            if index > max {
                maxt = (index, v)
            }
        }
    }

    println!("{}, {:?}, {:?}", line, mint, maxt);
    let out = (mint.1.to_string() + &maxt.1.to_string()).to_string().parse::<i32>().unwrap();
    println!("{}", out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_part_one() {
        assert_eq!(calibration_values("1abc2".to_string(), &NUMERICAL_VALUES.to_vec()), 12);
        assert_eq!(calibration_values("2sdafs3".to_string(), &NUMERICAL_VALUES.to_vec()), 23);
    }

    #[test]
    fn test_single_line_part_two() {
        let mut all_values = Vec::new();
        all_values.extend_from_slice(&TEXT_VALUES);
        all_values.extend_from_slice(&NUMERICAL_VALUES);

        assert_eq!(calibration_values("dfjnzxtlnine9five".to_string(), &all_values), 95);
        assert_eq!(calibration_values("eightwothree".to_string(), &all_values), 83);
        assert_eq!(calibration_values("abcone2threexyz".to_string(), &all_values), 13);
    }

    #[test]
    fn test_example_part_one() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part_one(example_input.to_string()), 142);
    }

    #[test]
    fn test_example_part_two() {
        println!("hello!");
        let example_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part_two(example_input.to_string()), 281);
    }
}
