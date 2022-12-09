use std::collections::HashSet;
use std::fs;

fn read_input(path: &str) -> Vec<((i64, i64), i64)> {
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut it = line.split(' ');
            let dir = dbg!(it.next().unwrap());
            let val = dbg!(it.next().unwrap()).parse::<i64>().unwrap();
            match dir {
                "U" => ((0, 1), val),
                "R" => ((1, 0), val),
                "D" => ((0, -1), val),
                "L" => ((-1, 0), val),
                _ => panic!("wtf?"),
            }
        })
        .collect::<Vec<_>>()
}

fn advance(h: (i64, i64), t: (i64, i64)) -> (i64, i64) {
    match (h.0 - t.0, h.1 - t.1) {
        (i, 0) if i.abs() == 2 => (t.0 + i / i.abs(), t.1),
        (0, i) if i.abs() == 2 => (t.0, t.1 + i / i.abs()),
        (i, j) if [j.abs(), i.abs()].contains(&2) => (t.0 + i / i.abs(), t.1 + j / j.abs()),
        (x, y) if x.abs() < 2 && y.abs() < 2 => (t.0, t.1),
        (x, y) => panic!("unhandled case for diff {},{}", x, y),
    }
}

pub fn p1(input_path: &str) -> i64 {
    let mut visited = HashSet::<(i64, i64)>::new();
    let mut h = (0, 0);
    let mut t = (0, 0);
    visited.insert(t);
    for (step, val) in read_input(input_path) {
        dbg!(step, val);
        for _ in 0..val {
            h.0 += step.0;
            h.1 += step.1;
            t = advance(h, t);
            match visited.insert(t) {
                true => {
                    dbg!(t);
                }
                false => {}
            }
        }
    }

    visited.len() as i64
}

pub fn p2(input_path: &str) -> i64 {
    let mut visited = HashSet::<(i64, i64)>::new();
    let mut ts = (0..10).map(|_| (0, 0)).collect::<Vec<_>>();
    visited.insert(ts[0]);
    for (step, val) in read_input(input_path) {
        dbg!(step, val);
        for _ in 0..val {
            ts[0].0 += step.0;
            ts[0].1 += step.1;

            for i in 0..ts.len() - 1 {
                let h = ts[i];
                let t = &mut ts[i + 1];
                *t = advance(h, *t);
            }
            let t = &ts[9];
            visited.insert(*t);
        }
    }

    visited.len() as i64
}

#[cfg(test)]
mod test {
    use crate::d9::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d9_test.txt"), 13);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d9_test.txt"), 1);
        assert_eq!(p2("input/d9_test2.txt"), 36);
    }

    #[test]
    fn p1_task() {
        assert_eq!(p1("input/d9.txt"), 6190);
    }

    #[test]
    fn p2_task() {
        assert_eq!(p2("input/d9.txt"), 2516);
    }
}
