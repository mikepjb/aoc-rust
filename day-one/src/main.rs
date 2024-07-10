fn main() {
    println!("{}", part_one("2fsdaf5".to_string()));
}

fn part_one(input: String) -> String {
    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

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
