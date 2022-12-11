use std::fs;
use std::collections::VecDeque;
use itertools;

use itertools::Itertools;

#[derive(Debug)]
enum Cmd {
    Add(i64),
    Nop,
}

#[derive(Debug)]
enum Op {
    Val(i64),
    Old
}

#[derive(Debug)]
struct Monkey {
    id: i64,
    items: VecDeque<i64>,
    op: char,
    op_val: Op,
    test_div: i64,
    true_target: usize,
    false_target: usize,
    thrown: i64,
}

fn parse_monkey(text: &str) -> Monkey {
    // let id = Regex::new(r"Monkey (\d+):").unwrap();
    // let op = Regex::new(r"old (.) (\d+)").unwrap();
    // let test = Regex::new(r"divisible by (\d+)").unwrap();
    // let target = Regex::new(r"to monkey (d\+)");

    // let mut it = text.split("\n");
    // let i = id.captures(it.next().unwrap()).unwrap();
    let mut lines = text.split('\n');
    let mut id_chars = lines.next().unwrap().trim_start().split(' ').skip(1).next().unwrap().chars();
    id_chars.next_back().unwrap();
    let id = id_chars.as_str().parse::<i64>().unwrap();
    let items = lines.next().unwrap().trim_start().split(": ").skip(1).next().unwrap().split(", ").map(|v| v.parse::<i64>().unwrap()).collect::<VecDeque<i64>>();
    let mut op_it = lines.next().unwrap().trim_start().split(' ').skip(4);
    let op = op_it.next().unwrap().parse::<char>().unwrap();
    let op_val = match op_it.next().unwrap() {
        "old" => Op::Old,
        v => Op::Val(v.parse::<i64>().unwrap()),
    };
    let test_div = lines.next().unwrap().trim_start().split(' ').skip(3).next().unwrap().parse::<i64>().unwrap();
    let true_target = lines.next().unwrap().trim_start().split(' ').skip(5).next().unwrap().parse::<usize>().unwrap();
    let false_target = lines.next().unwrap().trim_start().split(' ').skip(5).next().unwrap().parse::<usize>().unwrap();

    Monkey{
        id,
        items,
        op,
        op_val,
        test_div,
        true_target,
        false_target,
        thrown: 0,
    }
}

fn read_input(path: &str) -> Vec<Monkey> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|monkey_text| {
            parse_monkey(monkey_text)
        })
        .collect()
}

pub fn p1(input_path: &str) -> i64 {
    let mut  monkeys = read_input(input_path);

    for _round in 0..20{
        for id in 0..monkeys.len() {
            while !monkeys[id].items.is_empty() {
                let item = monkeys[id].items.pop_front().unwrap();
                let op_val = match monkeys[id].op_val { Op::Old => item, Op::Val(v) => v,};
                let new_val = match monkeys[id].op {
                    '*' => item * op_val,
                    '+' => item + op_val,
                    _ => panic!("unknown operation"),
                } / 3;
                let target = if new_val % monkeys[id].test_div == 0 {
                    monkeys[id].true_target
                } else {
                    monkeys[id].false_target
                };

                monkeys[target].items.push_back(new_val);
                monkeys[id].thrown += 1;
            }
        }
        println!("cycle {} ", _round);
        for id in 0..monkeys.len() {
            println!("  {}: {}", id, monkeys[id].items.iter().cloned().map(|v| format!("{}", v)).join(","));
        }
    }
    dbg!(&monkeys);

    monkeys.iter().map(|m| m.thrown).sorted().rev().take(2).fold(1, |acc, x| acc * x)
}

pub fn p2(input_path: &str) -> i64 {
    let mut  monkeys = read_input(input_path);
    let divider = monkeys.iter().map(|m| m.test_div).fold(1, |acc, x| acc * x);

    for _round in 0..10000{
        for id in 0..monkeys.len() {
            while !monkeys[id].items.is_empty() {
                let item = monkeys[id].items.pop_front().unwrap();
                let op_val = match monkeys[id].op_val { Op::Old => item, Op::Val(v) => v,};
                let new_val = match monkeys[id].op {
                    '*' => item * op_val ,
                    '+' => item + op_val,
                    _ => panic!("unknown operation"),
                } % divider;
                let target = if new_val % monkeys[id].test_div == 0 {
                    monkeys[id].true_target
                } else {
                    monkeys[id].false_target
                };

                monkeys[target].items.push_back(new_val);
                monkeys[id].thrown += 1;
            }
        }
        if (_round +1 )% 1000 ==0 {
            println!("cycle {} ", _round+1);
            for id in 0..monkeys.len() {
                println!("  {}: {}", id, monkeys[id].thrown);
            }
        }
    }

    monkeys.iter().map(|m| m.thrown).sorted().rev().take(2).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod test {
    use crate::d11::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d11_test.txt"), 10605);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d11_test.txt"), 2713310158);
    }

    #[test]
    fn p1_task() {
        assert_eq!(p1("input/d11.txt"), 54253);
    }

    #[test]
    fn p2_task() {
        assert_eq!(p2("input/d11.txt"), 13119526120);
    }

}
