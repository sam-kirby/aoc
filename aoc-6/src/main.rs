use aoc_lib::load_split_input;

use std::collections::HashSet;

fn solve1(answers: &Vec<String>) -> usize {
    answers
        .iter()
        .map(|s| {
            s.chars()
                .filter(|&c| c != '|')
                .fold(HashSet::new(), |mut s, c| {
                    s.insert(c);
                    s
                })
                .len()
        })
        .sum()
}

fn solve2(answers: &Vec<String>) -> usize {
    answers
        .iter()
        .map(|s| {
            let mut set = Vec::new();
            for e in s.split("|") {
                if set.is_empty() {
                    for c in e.chars() {
                        set.push(c);
                    }
                } else {
                    set = set.into_iter().partition::<Vec<_>, _>(|&c| e.contains(c)).0;
                }

                if set.is_empty() {
                    break;
                }
            }
            set.len()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let answers: Vec<Result<String, _>> = load_split_input("inputs/6.txt", Some("|"))?;
    let answers = answers.into_iter().map(Result::unwrap).collect::<Vec<_>>();

    println!(
        "{} answers were given by at least one member of a group",
        solve1(&answers)
    );

    println!(
        "{} answers were given by every member of a group",
        solve2(&answers)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_lib::load_split_input;

    use super::{solve1, solve2};

    #[test]
    fn test1() {
        let answers: Vec<Result<String, _>> = load_split_input("test.txt", None).unwrap();
        let answers = answers.into_iter().map(Result::unwrap).collect::<Vec<_>>();

        assert_eq!(solve1(&answers), 11);
    }

    #[test]
    fn test2() {
        let answers: Vec<Result<String, _>> = load_split_input("test.txt", Some("|")).unwrap();
        let answers = answers.into_iter().map(Result::unwrap).collect::<Vec<_>>();

        assert_eq!(solve2(&answers), 6);
    }
}
