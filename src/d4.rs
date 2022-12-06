use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(path: &'static str) -> std::io::Lines<BufReader<File>> {
    BufReader::new(File::open(path).unwrap()).lines()
}

fn encloses(outer: &Vec<i64>, inner: &Vec<i64>) -> bool
{
    // println!("{}-{},{}-{}", outer[0], outer[1], inner[0], inner[1]);
    assert!(outer.len() == inner.len() && outer.len() == 2);
    assert!(outer[0] <= outer[1] && inner[0] <= inner[1]);
    outer[0] <= inner[0] && outer[1] >= inner[1]
}



fn overlap(s0: &Vec<i64>, s1: &Vec<i64>) -> bool
{
    // println!("{}-{},{}-{}", s0[0], s0[1], s1[0], s1[1]);
    assert!(s0.len() == s1.len() && s0.len() == 2);
    assert!(s0[0] <= s0[1] && s1[0] <= s1[1]);
    s0[0] <= s1[0] && s0[1] >= s1[0]
}

pub fn p1() -> i64 {
    dbg!(read_lines("input/d4.txt").map(
        |l| l
            .unwrap()
            .split(",")
            .map(|s| s
                .split("-")
                .map(|sid| sid.parse::<i64>().unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<Vec<i64>>>()
    )
        .map( |sections| encloses(&sections[0], &sections[1]) || encloses(&sections[1], &sections[0]))
        .filter(|is_enclosing| *is_enclosing)
    )
    .count() as i64
}

pub fn p2() -> i64 {
    dbg!(read_lines("input/d4.txt").map(
        |l| l
            .unwrap()
            .split(",")
            .map(|s| s
                .split("-")
                .map(|sid| sid.parse::<i64>().unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<Vec<i64>>>()
    )
        .map( |sections| overlap(&sections[0], &sections[1]) || overlap(&sections[1], &sections[0]))
        .filter(|is_enclosing| *is_enclosing)
    )
    .count() as i64
}

#[cfg(test)]
mod tests {
    use crate::d4::p2;
    #[test]
    fn p1_test() {
        assert_eq!(p2(), 2681);
    }
}
