use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Gear Ratios");
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn simple_test() {
        assert!(true)
    }
}
