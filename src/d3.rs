use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn p1() -> i64 {
    let file_reader = BufReader::new(File::open("input/d3.txt").unwrap());
    file_reader
        .lines()
        .map(|line| {
            let s = line.unwrap();
            let half = s.len() / 2;
            (
                s.chars().take(half).collect::<HashSet<char>>(),
                s.chars().skip(half).take(half).collect::<HashSet<char>>(),
            )
        })
        .map(|(a, b)| {
            a.intersection(&b)
                .map(|c| match c {
                    c if c >= &'a' && c <= &'z' => *c as i64 - 'a' as i64 + 1,
                    c if c >= &'A' && c <= &'Z' => *c as i64 - 'A' as i64 + 27,
                    _ => panic!("unexpected input"),
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn p2() -> i64 {
    let file_reader = BufReader::new(File::open("input/d3.txt").unwrap());
    file_reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<HashSet<char>>())
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let a = chunk.next().unwrap();
            let b = chunk.next().unwrap();
            let c = chunk.next().unwrap();
            a.intersection(&b)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&c)
                .map(|c| match c {
                    c if c >= &'a' && c <= &'z' => *c as i64 - 'a' as i64 + 1,
                    c if c >= &'A' && c <= &'Z' => *c as i64 - 'A' as i64 + 27,
                    _ => panic!("unexpected input"),
                })
                .sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::d3::{p1, p2};
    #[test]
    fn p1_test() {
        assert_eq!(p1(), 8349);
        assert_eq!(p2(), 2681);
    }
}
