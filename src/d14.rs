use std::fs;
use std::cmp::{min, max};
use std::collections::VecDeque;

type Point = (usize, usize);

fn read_input(path: &str) -> Vec<Vec<Point>> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(" -> ").map(|cord_str| {
                let mut it  = cord_str.split(",");
                (it.next().unwrap().parse::<usize>().unwrap(), it.next().unwrap().parse::<usize>().unwrap())
            }).collect::<Vec<Point>>()
        })
        .collect()
}

struct Canvas {
    v: VecDeque<VecDeque<char>>,
    source: Point,
}

fn create_canvas(lines: Vec<Vec<Point>>) -> Canvas {
    let max_x = *lines.iter().flatten().map(|(x, _)| x).max().unwrap();
    let min_x = *lines.iter().flatten().map(|(x, _)| x).min().unwrap();
    let max_y = *lines.iter().flatten().map(|(_, y)| y).max().unwrap();
    let min_y = 0;//*lines.iter().flatten().map(|(_, y)| y).min().unwrap();
    let stride = max_x - min_x + 1;
    let line_len = max_y - min_y +1;
    let mut v: VecDeque<VecDeque<char>> = vec![vec!['.'; stride].into_iter().collect::<VecDeque<char>>(); line_len].into_iter().collect();

    for line in lines {
        for ((sx, sy), (ex, ey)) in line.iter().zip(line.iter().skip(1)) {
            if sx == ex {
                let s = *min(sy, ey);
                let e = *max(sy, ey);

                for i in s..e+1 {
                    v[(i - min_y)][(sx - min_x)] = '#';
                }
            } else if sy == ey {
                let s = *min(sx, ex);
                let e = *max(sx, ex);

                for i in s..e+1 {
                    v[(sy-min_y)][i - min_x] = '#';
                }
            }
        }
    }
    Canvas {
        v,
        source: (500 - min_x, 0)
    }
}

fn print_canvas(c: &Canvas){
    for l in c.v.iter() {
        println!();
        for ch in l {
            print!("{}", ch);
        }
    }
    println!();
}

fn simulate(c: &mut Canvas) {
    let spawn_index = c.source;
    'sim: loop {
        if c.v[c.source.1][c.source.0] != '.' { break 'sim; };
        let mut cur = spawn_index;
        c.v[c.source.1][c.source.0] = 'o';
        'grain: loop {
            let mut moved = false;
            'step: for n in [(cur.0 as i64, cur.1+1), ((cur.0 as i64-1) , cur.1+1), ((cur.0 as i64 +1), cur.1+1)] {
                if n.0 < 0 || n.0 as usize > c.v[0].len() || n.1 > c.v.len(){ // out of bounds
                    c.v[cur.1][cur.0] = '.';
                    break 'sim;
                }
                if c.v[(n.1)][(n.0 as usize)] == '.' {
                    (c.v[n.1][n.0 as usize], c.v[cur.1][cur.0]) = (c.v[cur.1][cur.0], c.v[n.1][n.0 as usize]);
                    cur.0 = n.0 as usize;
                    cur.1 = n.1;
                    moved = true;
                    break 'step;
                }
            }
            if ! moved {
                break 'grain;
            }
        };
        // print_canvas(&c);
    }
}

pub fn p1(path: &str) -> i64 {
    let input = read_input(path);
    let mut c = create_canvas(input);
    simulate(&mut c);
    print_canvas(&c);
    c.v.iter().flatten().filter(|c| **c=='o').count() as i64
}

fn simulate2(c: &mut Canvas) {
    'sim: loop {
        if c.v[c.source.1][c.source.0] != '.' { break 'sim; };
        let mut cur = c.source;
        c.v[c.source.1][c.source.0] = 'o';
        'grain: loop {
            let mut moved = false;
            'step: for mut n in [(cur.0 as i64, cur.1+1), ((cur.0 as i64-1) , cur.1+1), ((cur.0 as i64 +1), cur.1+1)] {
                if n.0 < 0 { // extend left;
                    n.0 += 1;
                    cur.0 += 1;
                    c.source.0 += 1;
                    for i in 0..c.v.len() {
                        c.v[i].push_front('.');
                    }
                }
                if n.0 as usize == c.v[0].len() { //extend right
                    for i in 0..c.v.len() {
                        c.v[i].push_back('.');
                    }
                }
                if (n.1) == (c.v.len()) { // out of bounds
                    c.v[cur.1][cur.0] = 'o';
                    break 'grain;
                }
                (c.v.len(), c.v[0].len());
                if c.v[(n.1)][(n.0 as usize)] == '.' {
                    (c.v[n.1][n.0 as usize], c.v[cur.1][cur.0]) = (c.v[cur.1][cur.0], c.v[n.1][n.0 as usize]);
                    cur.0 = n.0 as usize;
                    cur.1 = n.1;
                    moved = true;
                    break 'step;
                }
            }
            if ! moved {
                break 'grain;
            }
        };
    }
}


pub fn p2(path: &str) -> i64 {
    let input = read_input(path);
    let mut c = create_canvas(input);
    c.v.push_back(vec!['.'; c.v[0].len()].into_iter().collect());
    simulate2(&mut c);
    print_canvas(&c);
    c.v.iter().flatten().filter(|c| **c=='o').count() as i64
}

#[cfg(test)]
mod test {
    use crate::d14::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d14_test.txt"), 24);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d14_test.txt"), 93);
    }

    // #[test]
    // fn p1_task() {
    //     assert_eq!(p1("input/d14.txt"), 5852);
    // }

    #[test]
    fn p2_task() {
        assert_eq!(p2("input/d14.txt"), 24190);
    }
}
