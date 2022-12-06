use itertools::Itertools;
use std::fs;
fn initial_state(desc: String) -> Vec<Vec<char>> {
    let v = desc
        .split('\n')
        .map(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|mut chunk| {
                    chunk.next();
                    match chunk.next().unwrap() {
                        ' ' => None,
                        a if a >= '0' && a <= '9' => None,
                        a => Some(a.clone()),
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Vec<Option<char>>>>();

    let len = v[0].len();
    let transposed = (0..len)
        .map(|i| {
            let mut it = v.clone().into_iter().flatten();
            for _ in 0..i {
                it.next();
            }
            it.step_by(len)
                .filter_map(|x| x)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect()
        })
        .collect();

    transposed
}

pub fn p1() -> i64 {
    let binding = fs::read_to_string("input/d5.txt").unwrap();
    let mut parts = binding.split("\n\n");
    let desc = String::from(parts.next().unwrap());
    let moves = String::from(parts.next().unwrap());
    let mut state = dbg!(initial_state(desc));

    moves.split("\n").map(|line| {
        line.split(' ')
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }).for_each(|m| {
        dbg!(m.clone());
        for _ in 0..m[0] {
            let moved = state[m[1] - 1].pop().unwrap();
            state[m[2] - 1].push(moved);
        }
        dbg!(state.clone());
    });

    dbg!(state.iter().map(|v| v.last().unwrap()).cloned().collect::<Vec<char>>());

    0
}

pub fn p2() -> i64 {
    let binding = fs::read_to_string("input/d5.txt").unwrap();
    let mut parts = binding.split("\n\n");
    let desc = String::from(parts.next().unwrap());
    let moves = String::from(parts.next().unwrap());
    let mut state = dbg!(initial_state(desc));

    moves.split("\n").map(|line| {
        line.split(' ')
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }).for_each(|m| {
        dbg!(m.clone());

            let len = state[m[1] - 1].len();
        
            let mut to_move: Vec<char> = state[m[1] - 1].drain(len - m[0]..).collect();
            state[m[2] - 1].append(&mut to_move);
        dbg!(state.clone());
    });

    dbg!(state.iter().map(|v| v.last().unwrap()).cloned().collect::<Vec<char>>());

    0
}

#[cfg(test)]
mod tests {
    use crate::d5::p1;
    #[test]
    fn p1_test() {
        assert_eq!(p1(), 2681);
    }
}
