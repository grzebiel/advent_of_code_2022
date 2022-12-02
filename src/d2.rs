use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn resolve_strategy(f: &dyn Fn(String) -> i64) -> i64
{
    let file_reader = BufReader::new(File::open("input/d2.txt").unwrap());
    file_reader
        .lines()
        .map(|line| line.unwrap())
        .map(f)
        .sum()
}

pub fn p1() -> i64
{
    resolve_strategy(&|round: String|
            match round.split(" ").collect::<Vec<&str>>()[..] {
                ["A", "X"] => 1 + 3,
                ["A", "Y"] => 2 + 6,
                ["A", "Z"] => 3 + 0,

                ["B", "X"] => 1 + 0,
                ["B", "Y"] => 2 + 3,
                ["B", "Z"] => 3 + 6,

                ["C", "X"] => 1 + 6,
                ["C", "Y"] => 2 + 0,
                ["C", "Z"] => 3 + 3,

                _ => panic!("to many items in row")
            }
        )
}

pub fn p2() -> i64
{
    resolve_strategy(&|round: String|
            match round.split(" ").collect::<Vec<&str>>()[..] {
                ["A", "X"] => 3 + 0,
                ["A", "Y"] => 1 + 3,
                ["A", "Z"] => 2 + 6,

                ["B", "X"] => 1 + 0,
                ["B", "Y"] => 2 + 3,
                ["B", "Z"] => 3 + 6,

                ["C", "X"] => 2 + 0,
                ["C", "Y"] => 3 + 3,
                ["C", "Z"] => 1 + 6,

                _ => panic!("to many items in row")
            }
        )
}
