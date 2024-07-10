// use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let day_input = fs::read_to_string("./input/day-one.txt")?;
    // println!("{}", part_one(day_input));
    calibration_values("hello".to_string());
    part_one("hello".to_string());
    Ok(())
}

fn part_one(_input: String) -> i32 {
    32
}

fn calibration_values(_line: String) -> i32 {
    32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_part_one() {
        assert_eq!(calibration_values("1abc2".to_string()), 12);
        assert_eq!(calibration_values("2sdafs3".to_string()), 23);
        assert_eq!(calibration_values("2sdaf3s".to_string()), 23);
        assert_eq!(calibration_values("dfjnzxtlnine9five".to_string()), 95);
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
        let example_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part_one(example_input.to_string()), 281);
    }
}
