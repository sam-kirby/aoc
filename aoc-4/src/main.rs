use aoc_lib::load_split_input;

use std::error::Error;

mod passport;
use passport::Passport;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let passports: Vec<Result<Passport, _>> = load_split_input("inputs/4.txt", Some(" "))?;

    println!(
        "Passports valid in pass 1: {}",
        passports.iter().filter_map(|r| r.as_ref().ok()).count()
    );

    println!(
        "Passports valid in pass 2: {}",
        passports
            .iter()
            .filter_map(|r| r.as_ref().ok())
            .filter(|&p| p.passes_enhanced_validation())
            .count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Passport;
    use aoc_lib::load_split_input;

    #[test]
    fn test1() {
        let passports: Vec<Result<Passport, _>> =
            load_split_input("tests/test.txt", Some(" ")).unwrap();
        assert_eq!(passports.iter().filter_map(|r| r.as_ref().ok()).count(), 2);
    }

    #[test]
    fn test_enhanced_valid() {
        let passports: Vec<Result<Passport, _>> =
            load_split_input("tests/test_enhanced_valid.txt", Some(" ")).unwrap();
        assert_eq!(
            passports
                .iter()
                .filter_map(|r| r.as_ref().ok())
                .filter(|&p| p.passes_enhanced_validation())
                .count(),
            4
        )
    }

    #[test]
    fn test_enhanced_invalid() {
        let passports: Vec<Result<Passport, _>> =
            load_split_input("tests/test_enhanced_invalid.txt", Some(" ")).unwrap();
        assert_eq!(
            passports
                .iter()
                .filter_map(|r| r.as_ref().ok())
                .filter(|&p| p.passes_enhanced_validation())
                .count(),
            0
        )
    }
}
