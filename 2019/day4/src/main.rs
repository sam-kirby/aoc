use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = env::args().collect::<Vec<_>>();
    let lower_bound = arguments[1].trim().parse::<usize>()?;
    let upper_bound = arguments[2].trim().parse::<usize>()?;

    let possible_passwords = (lower_bound..=upper_bound)
        .filter(|candidate| {
            let digits = {
                let mut candidate = *candidate;
                let mut result = Vec::new();
                while candidate > 0 {
                    result.push(candidate % 10);
                    candidate /= 10;
                }
                result.reverse();
                result
            };

            // test length
            if digits.len() < 6 {
                return false;
            }

            // test that there exists a pair of matching digits
            let has_pair = digits.iter().enumerate().fold(false, |out, digit| {
                if digit.0 == digits.len() - 1 || out {
                    return out;
                }
                *digit.1 == digits[digit.0 + 1]
            });
            if !has_pair {
                return false;
            }

            // test that digits increase going left to right
            let digits_decrease = digits.iter().enumerate().fold(false, |out, digit| {
                if digit.0 == digits.len() - 1 || out {
                    return out;
                }
                *digit.1 > digits[digit.0 + 1]
            });
            if digits_decrease {
                return false;
            };

            true
        })
        .collect::<Vec<_>>();

    println!("There are {} possible passwords", possible_passwords.len());

    let possible_passwords_only_doubles = possible_passwords
        .iter()
        .filter(|candidate| {
            let digits = {
                let mut candidate = **candidate;
                let mut result = Vec::new();
                while candidate > 0 {
                    result.push(candidate % 10);
                    candidate /= 10;
                }
                result.reverse();
                result
            };

            let digit_map = digits.iter().fold((Vec::new(), 0), |mut state, digit| {
                if *digit == state.1 {
                    let index = state.0.len() - 1;
                    state.0[index] += 1;
                } else {
                    state.0.push(1);
                }
                state.1 = *digit;
                state
            });

            if !digit_map.0.contains(&2) {
                return false;
            }

            true
        })
        .collect::<Vec<_>>();

    println!(
        "There are {} possible passwords where only doubles are allowed",
        possible_passwords_only_doubles.len()
    );

    Ok(())
}
