use aoc_lib::load_simple_input;

use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Seat(u16);

impl FromStr for Seat {
    type Err = SeatParseError;

    #[allow(unused_parens)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seat_num = 0;
        let mut chars = s.chars();
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'R') as u16);
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'R') as u16) << 1;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'R') as u16) << 2;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'B') as u16) << 3;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'B') as u16) << 4;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'B') as u16) << 5;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'B') as u16) << 6;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'B') as u16) << 7;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'B') as u16) << 8;
        seat_num |= ((chars.next_back().ok_or(SeatParseError::MissingDigit)? == 'B') as u16) << 9;

        Ok(Seat(seat_num))
    }
}

#[derive(Debug)]
enum SeatParseError {
    // InvalidCharacter(char),
    MissingDigit,
}

impl Display for SeatParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // SeatParseError::InvalidCharacter(c) => write!(f, "found an invalid char {}", c),
            SeatParseError::MissingDigit => write!(f, "too few digits"),
        }
    }
}

impl Error for SeatParseError {}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let occupied_seats: HashSet<Seat> = load_simple_input("inputs/5.txt")?;

    let max_id = occupied_seats.iter().max().unwrap().0;
    let min_id = occupied_seats.iter().min().unwrap().0;

    println!("The largest seat ID is: {}", max_id);

    let free_seat = {
        let mut free_seat = None;
        for maybe_free in min_id..max_id {
            if !occupied_seats.contains(&Seat(maybe_free)) {
                free_seat.replace(maybe_free);
                break;
            }
        }
        free_seat.unwrap()
    };
    println!("Your seat is: {}", free_seat);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Seat;
    use aoc_lib::load_simple_input;
    #[test]
    fn test1() {
        let seats: Vec<Seat> = load_simple_input("test.txt").unwrap();
        assert_eq!(seats.iter().max().unwrap().0, 820);
    }
}
