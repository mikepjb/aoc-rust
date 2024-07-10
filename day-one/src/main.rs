fn main() {
    println!("{}", part_one("2fsda5f".to_string()));
}

fn part_one(input: String) -> String {
    input.split("\n")
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
        if line.len() > 1 {
            let mut chars = line.chars();
            chars.next();
            chars.next_back();
            line = chars.as_str().to_string();
        }
    }

    if start_n == 0 {
        start_n = end_n
    } else if end_n == 0 {
        end_n = start_n
    };
    return (start_n.to_string() + &end_n.to_string()).to_string().parse::<i32>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_part_one() {
        assert_eq!(calibration_values("1abc2".to_string()), 12);
        assert_eq!(calibration_values("2sdafs3".to_string()), 23);
        assert_eq!(calibration_values("2sdaf3s".to_string()), 23);
    }

    #[test]
    fn test_example_part_one() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part_one(example_input.to_string()), "142");

        assert_eq!(true, true);
    }
}
