use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn read_input(path: &str) -> GraphRepr {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

type Index = (usize, usize);
type GraphRepr = Vec<Vec<char>>;

struct Graph {
    repr: GraphRepr,
    start: Index,
    end: Index,
}

fn get_start_end(repr: &GraphRepr) -> (Index, Index) {
    let row_len = repr[0].len();
    let start_i = repr
        .iter()
        .flatten()
        .enumerate()
        .find(|(_, c)| **c == 'S')
        .unwrap()
        .0;
    let end_i = repr
        .iter()
        .flatten()
        .enumerate()
        .find(|(_, c)| **c == 'E')
        .unwrap()
        .0;

    (
        (dbg!(start_i) / row_len, start_i % row_len),
        (end_i / row_len, end_i % row_len),
    )
}

impl Graph {
    fn new(mut repr: GraphRepr) -> Graph {
        let (start, end) = get_start_end(&repr);
        repr[start.0][start.1] = 'a';
        repr[end.0][end.1] = 'z';
        Graph { repr, start, end }
    }

    fn nodes_count(self) -> usize {
        return self.repr.len() * self.repr[0].len();
    }

    fn get_possible_steps(&self, node: Index) -> Vec<Index> {
        let mut ret = vec![];
        for i in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
            let next = (node.0 as i32 + i.0, node.1 as i32 + i.1);
            if next.0 < 0
                || next.1 < 0
                || next.0 >= self.repr.len() as i32
                || next.1 >= self.repr[0].len() as i32
            {
                continue;
            }
            let cur_high = self.repr[node.0][node.1] as i32;

            let next_high = self.repr[next.0 as usize][next.1 as usize] as i32;

            let high_diff = next_high - cur_high;
            if high_diff > 1 {
                continue;
            }
            ret.push((next.0 as usize, next.1 as usize));
        }

        ret
    }

    fn print_visited(&self, visited: &HashSet<Index>) {
        println!("  01234567");
        println!(" +--------");
        for l in 0..self.repr.len() {
            print!("{:02}|", l);
            for c in 0..self.repr[0].len() {
                if visited.contains(&(l, c)) {
                    print!("*");
                } else {
                    print!("{}", self.repr[l][c]);
                }
            }
            println!();
        }
    }

    fn print_backtrack(&self, backtrack: &HashMap<Index, Index>, start: Index, end: Index) {
        {
            let mut repr = self.repr.clone();
            let mut cur = end;
            loop {
                repr[cur.0][cur.1] = '*';
                cur = backtrack[&cur];
                if cur == start {
                    break;
                }
            }
            println!("  01234567");
            println!(" +--------");
            for l in 0..repr.len() {
                for c in 0..repr[0].len() {
                    print!("{}", repr[l][c]);
                }
                print!(" ");
                for c in 0..repr[0].len() {
                    print!("{}", self.repr[l][c]);
                }
                println!();
            }
        }
    }

    fn bfs(&self, start: Index, end: Index) -> Option<i64> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::<Index>::new();
        let mut distances = HashMap::new();
        let mut preds = HashMap::new();
        // distances.entry(start).or_insert(0);
        queue.push_back((start, 0));
        distances.insert(start, 0);

        while queue.len() != 0 {
            let (node, depth) = queue.pop_front().unwrap();
            if visited.contains(&node) {
                continue;
            }
            for child in self.get_possible_steps(node) {
                if visited.contains(&child) {
                    continue;
                }
                queue.push_back((child, depth + 1));
                preds.entry(child).or_insert(node);

                if child == end {
                    break;
                }
            }
            visited.insert(node);
        }
        if !preds.contains_key(&end) {
            // self.print_visited(&visited);
            return None;
        }

        // self.print_backtrack(&preds, start, end);
        let mut len = 0;
        let mut cur = end;
        while cur != start {
            cur = preds[&cur];
            len += 1;
        }
        Some(len)
    }

    fn get_all_valued(&self, sc: char) -> Vec<Index> {
        let len = self.repr[0].len();
        self.repr
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, c)| sc == **c)
            .map(|(i, _)| (i / len, i % len))
            .collect()
    }
}

pub fn p1(input_path: &str) -> i64 {
    let input = read_input(input_path);
    let graph = Graph::new(input);

    graph.bfs(graph.start, graph.end).unwrap()
}

pub fn p2(input_path: &str) -> i64 {
    let input = read_input(input_path);
    let graph = Graph::new(input);

    graph
        .get_all_valued('a')
        .iter()
        .map(|start_point| graph.bfs(*start_point, graph.end))
        .flatten()
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::d12::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d12_test.txt"), 31);
        assert!(false);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d12_test.txt"), 29);
    }

    #[test]
    fn p1_task() {
        assert_eq!(p1("input/d12.txt"), 447);
    }

    #[test]
    fn p2_task() {
        assert_eq!(p2("input/d12.txt"), 446);
    }
}
