use itertools;
use std::fs;

fn read_forest(path: &str) -> Vec<Vec<i64>> {
    let mut f: Vec<Vec<i64>> = fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c as i64 - '0' as i64)
                .into_iter()
                .collect()
        })
        .collect();
    // empty vector at the end of input
    f.pop();
    f
}

pub fn p1(input_path: &str) -> i64 {
    let f = read_forest(input_path);
    let mut vis: Vec<Vec<bool>> = f
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();

    // l -> r pass
    for y in 0..f.len() {
        let mut highest = -1;
        for x in 0..f[0].len() {
            if f[y][x] > highest {
                vis[y][x] = true;
                highest = f[y][x];
            }
        }
    }

    // r -> l pass
    for y in 0..f.len() {
        let mut highest = -1;
        for x in (0..f[0].len()).rev() {
            if f[y][x] > highest {
                vis[y][x] = true;
                highest = f[y][x];
            }
        }
    }

    // t -> b pass
    for x in 0..f[0].len() {
        let mut highest = -1;
        for y in 0..f.len() {
            if f[y][x] > highest {
                vis[y][x] = true;
                highest = f[y][x];
            }
        }
    }

    // b -> p pass
    for x in 0..f[0].len() {
        let mut highest = -1;
        for y in (0..f.len()).rev() {
            if f[y][x] > highest {
                vis[y][x] = true;
                highest = f[y][x];
            }
        }
    }

    vis.into_iter().flatten().filter(|x| *x).count() as i64
}

fn calculate_score(f: &Vec<Vec<i64>>, scores: &mut Vec<Vec<(i64, i64, i64, i64)>>) {
    // l -> r pass
    let max_height = 9;
    for y in 0..f.len() {
        let mut last_tree_lookup = vec![0; max_height+1];
        for x in 0..f[0].len() {
            let first_colistion = *last_tree_lookup[f[y][x] as usize..last_tree_lookup.len() as usize].iter().max().unwrap() as usize;
            scores[y][x].0 = (x - first_colistion) as i64;
            last_tree_lookup[f[y][x] as usize] = x;
        }
    }

    // r -> l pass
    for y in 0..f.len() {
        let mut last_tree_lookup = vec![f[0].len() - 1; max_height+1];
        for x in (0..f[0].len()).rev() {
            let first_colistion = *last_tree_lookup[f[y][x] as usize..last_tree_lookup.len() as usize].iter().min().unwrap() as usize;
            scores[y][x].1 = (first_colistion - x) as i64;
            last_tree_lookup[f[y][x] as usize] = x;
        }
    }

    // t -> b pass
    for x in 0..f[0].len() {
        let mut last_tree_lookup = vec![0; max_height+1];
        for y in 1..f.len() {
            let first_colistion = *last_tree_lookup[f[y][x] as usize..last_tree_lookup.len() as usize].iter().max().unwrap() as usize;
            scores[y][x].2 = (y - first_colistion) as i64;
            last_tree_lookup[f[y][x] as usize] = y;
        }
    }

    // b -> p pass
    for x in 0..f[0].len() {
        let mut last_tree_lookup = vec![f.len() - 1; max_height+1];
        for y in (0..f.len() -1).rev() {
            let first_colistion = *last_tree_lookup[f[y][x] as usize..last_tree_lookup.len() as usize].iter().min().unwrap() as usize;
            scores[y][x].3 = (first_colistion - y) as i64;
            last_tree_lookup[f[y][x] as usize] = y;
        }
    }
}

pub fn p2(path: &str) -> i64 {
    let f = read_forest(path);
    let mut score: Vec<Vec<(i64, i64, i64, i64)>> = f
        .iter()
        .map(|row| row.iter().map(|_| (0, 0, 0, 0)).collect())
        .collect();
    calculate_score(&f, &mut score);
    let ret = score
        .iter()
        .flatten()
        .map(|(a, b, c, d)| a * b * c * d)
        .enumerate()
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    ret.1
}

#[cfg(test)]
mod test {
    use crate::d8::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d8_test.txt"), 21);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d8_test.txt"), 8);
    }

    #[test]
    fn p1_task() {
        dbg!(p1("input/d8.txt"));
    }

    #[test]
    fn p2_task() {
        dbg!(p2("input/d8.txt"));
        assert!(false)
    }
}
