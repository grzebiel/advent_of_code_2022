use std::io::{BufRead, BufReader};
use std::fs::File;

fn extract_elves_summed_callories() -> Vec<i64>
{
    let file_reader = BufReader::new(File::open("input/d1.txt").unwrap());
    file_reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap_or(-1))
        .collect::<Vec<i64>>()
        .split(|i| *i == -1)
        .map(|single| single.iter().sum())
        .collect()

}

pub fn p1() -> i64
{
    extract_elves_summed_callories()
        .into_iter().max().unwrap()
}

pub fn p2() -> i64
{
    let mut unsorted = extract_elves_summed_callories();
    unsorted.sort();
    dbg!(unsorted
        .into_iter()
        .rev()
        .take(3))
        .sum()
}
