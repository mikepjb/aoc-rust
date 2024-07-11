use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Gear Ratios");
    Ok(())
}

fn engine_parts(engine_schematic: String) -> Vec<usize> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert!(true)
    }

    #[test]
    fn small_engine_test() {
        let small_engine = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(
            engine_parts(small_engine.to_string()),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        )
    }
}
