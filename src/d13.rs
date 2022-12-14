use std::fs;
use std::num::ParseIntError;
use std::str::{FromStr, Chars};
use itertools::{self, Itertools};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Entity {
    List(Vec<Entity>),
    Val(i64),
}

fn parse_entity_val(s: &[u8]) -> (Entity, usize) {
    let mut tmp_s = String::new();
    let mut offset = 0;
    for b in s {
        offset += 1;
        let c = *b as char;
        match c {
            c if c.is_digit(10) => {tmp_s.push(c);},
            ',' => {break},
            ']' => {offset -= 1; break},
            c => unimplemented!("{}", c),
        }
    }
    (Entity::Val(tmp_s.parse::<i64>().unwrap()), offset)
}

fn parse_entity_list(s: &[u8]) -> (Entity, usize) {
    let mut ents = Vec::new();
    let mut i = 1 as usize;
    loop {
        let c = s[i] as char;
        match c {
            ']' => {i+= 1; break;},
            c if c.is_digit(10) => {let (val, offset) = parse_entity_val(&s[i..]); ents.push(val); i += offset},
            '[' => {let (val, offset) = parse_entity_list(&s[i..]); ents.push(val); i += offset},
            ',' => {i += 1},
            _ => unimplemented!()
        }
    }

    (Entity::List(ents), i)
}

impl FromStr for Entity {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut it = s.chars();
        // assert_eq!(it.next().unwrap(), '[');
        let (val, _) = parse_entity_list(s.as_bytes());
        Ok(val)
    }
}

fn read_input(path: &str) -> Vec<(Entity, Entity)> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|packets| {
            let mut it = packets.split("\n");
            let l = it.next().unwrap().parse::<Entity>().unwrap();
            let r = it.next().unwrap().parse::<Entity>().unwrap();
            (l, r)
        })
        .collect()
}

impl std::cmp::Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        in_order(self, other)
    }
}

impl std::cmp::PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(in_order(self, other))
    }
}

impl std::cmp::PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        in_order(self, other) == Ordering::Equal
    }
}

impl std::cmp::Eq for Entity {
}

fn in_order(l: &Entity, r:&Entity) -> Ordering {
    match (l, r) { 
        (Entity::List(lv), Entity::List(rv)) => {
            for (a, b) in lv.iter().zip(rv.iter()) {
                let ord = in_order(a, b);
                if ord == Ordering::Equal {
                    continue;
                }
                return ord
            }
            return lv.len().cmp(&rv.len());
        },
        (Entity::Val(lv), Entity::Val(rv)) => lv.cmp(rv),
        (Entity::Val(lv), Entity::List(rv)) => in_order(&Entity::List(vec![Entity::Val(*lv)]), r),
        (Entity::List(lv), Entity::Val(rv)) => in_order(l, &Entity::List(vec![Entity::Val(*rv)])),
        (_, _) => unimplemented!()
    }
}

pub fn p1(path: &str) -> i64 {
    let input = read_input(path);
    input.iter().enumerate().map(|(i, p)| (i+1, p)).filter(|(_, (l, r))| in_order(l, r) == Ordering::Less).map(|(i, _)| i as i64).sum()
}

fn print_entity(e: &Entity) {
    match e {
        Entity::Val(v) => {print!("{v}")},
        Entity::List(v) => {print!("[");for ev in v {print_entity(ev); print!(",");}; print!("]")},
    }
}
fn print_stream(stream: &Vec<Entity>) {
    for (i, e) in stream.iter().enumerate() {
        print!("{:3}: ", i+1);
        print_entity(&e);
        println!();
    }
}
fn p2(path: &str) -> i64 {
    let input = read_input(path);
    let new_ones = vec![Entity::List(vec![Entity::List(vec![Entity::Val(2)])]), Entity::List(vec![Entity::List(vec![Entity::Val(6)])])];
    let it1 = input.iter().map(|(a, _)| a);
    let it2 = input.iter().map(|(_, a)| a);
    let sorted: Vec<_> = it1.interleave(it2).chain(new_ones.iter()).sorted().cloned().collect();
    let first = sorted.iter().position(|x| *x == new_ones[0]).unwrap();
    let second = sorted.iter().position(|x| *x == new_ones[1]).unwrap();
    // print_stream(&sorted);
    (1 + dbg!(first) as i64) * (1+ dbg!(second) as i64)
}

#[cfg(test)]
mod test {
    use crate::d13::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d13_test.txt"), 13);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d13_test.txt"), 140);
    }

    #[test]
    fn p1_task() {
        assert_eq!(p1("input/d13.txt"), 5852);
    }

    #[test]
    fn p2_task() {
        assert_eq!(p2("input/d13.txt"), 24190);
    }
}
