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
    let mut vis : Vec<Vec<bool>> = f.iter().map(|row| row.iter().map(|_| false).collect()).collect();

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

fn calculate_score(f: &Vec<Vec<i64>>, tx: usize, ty: usize) -> i64 {
    // l->r pass
    dbg!(f[ty][tx]);
    let (mut lrs, mut rls, mut tbs, mut bts) = (0,0,0,0);
    for x in tx+1..f[0].len() {
        lrs += 1;
        if dbg!(f[ty][x]) >= dbg!(f[ty][tx]) {
            break
        }
    }
    // r->l pass
    for x in (0..tx).rev() {
        rls += 1;
        if f[ty][x] >= f[ty][tx] {
            break
        }
    }
    // t->b pass
    for y in ty+1..f.len() {
        tbs += 1;
        if f[y][tx] >= f[ty][tx] {
            break
        }
    }
    // b->t pass
    for y in (0..ty).rev() {
        bts += 1;
        if f[y][tx] >= f[ty][tx] {
            break
        }
    }
    dbg!(lrs) * dbg!(rls) * dbg!(tbs) * dbg!(bts)
    // lrs * rls * tbs * bts
}

pub fn p2(path: &str) -> i64{
    let f = read_forest(path);
    let mut score : Vec<Vec<i64>> = f.iter().map(|row| row.iter().map(|_| 0).collect()).collect();
    for y in 0..f.len() {
        for x in 0..f[0].len() {
            score[y][x] = calculate_score(&f, x, y);
        }
    }
    let ret = score.into_iter().flatten().enumerate().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
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
    }
}
