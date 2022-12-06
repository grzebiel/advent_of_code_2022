use itertools;
use itertools::Itertools;
use itertools::izip;
use std::collections::HashSet;
use std::fs;
use std::iter::Zip;

pub fn p1() -> usize {
    let input = fs::read_to_string("input/d6.txt").unwrap();
    let i1 = input.chars();
    let i2 = input.chars().skip(1);
    let i3 = input.chars().skip(2);
    let i4 = input.chars().skip(3);

    for (i, (((a, b), c), d)) in i1.zip(i2).zip(i3).zip(i4).enumerate() {
        let set: HashSet<char> = [a, b, c, d].into_iter().collect();
        if set.len() == 4 {
            return i + 4;
        }
    }
    panic!("not found")
}

pub fn p2() -> usize {
    let input = fs::read_to_string("input/d6.txt").unwrap();
    let i1 = input.chars().collect_vec();

    for i in 0..i1.len()-14 {
        let set: HashSet<char> = i1[i..i+14].into_iter().cloned().collect();
        if set.len() == 14 {
            return i + 14;
        }
    }
    panic!("not found")
}
#[cfg(test)]
mod tests {
    use crate::d6::p2;
    #[test]
    fn p1_test() {
        dbg!(p2());
    }
}
