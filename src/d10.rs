use std::fs;

#[derive(Debug)]
enum Cmd {
    Add(i64),
    Nop,
}

fn read_input(path: &str) -> Vec<Cmd> {
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut it = line.split(' ');
            let cmd = it.next().unwrap();

            match cmd {
                "addx" => Cmd::Add(it.next().unwrap().parse::<i64>().unwrap()),
                "noop" => Cmd::Nop,
                _ => panic!("wtf?"),
            }
        })
        .rev()
        .collect()
}

pub fn p1(input_path: &str) -> i64 {
    let mut commands_queue = read_input(input_path);

    let mut to_add = None;
    let mut register = 1;
    let mut signal_sum = 0;
    let mut skip_cycle = false;
    for cycle in 1..221 {
        if (cycle - 20) % 40 == 0 {
            signal_sum += dbg!(dbg!(register) * dbg!(cycle));
        }

        if skip_cycle {
            skip_cycle = false;
        } else {
            match commands_queue.pop().unwrap() {
                Cmd::Add(v) => {
                    to_add = Some(v);
                    skip_cycle = true;
                }
                Cmd::Nop => {}
            }
            continue;
        }

        to_add.map_or((), |v| {
            register += v;
        });
    }
    signal_sum
}

pub fn render_crt(crt: &[char]) {
    println!("+----------------------------------------+");
    for (cycle, pixel) in crt.iter().enumerate() {
        if (cycle + 1) % 40 == 1 {
            print!("|");
        }
        print!("{}", pixel);

        if (cycle + 1) % 40 == 0 {
            println!("|");
        }
    }
    println!("+----------------------------------------+");
}

pub fn p2(input_path: &str) {
    let mut commands_queue = read_input(input_path);
    let mut crt = vec![' '; 6 * 40];

    let mut to_add = None;
    let mut register: i64 = 1;
    let mut skip_cycle = false;
    let mut cycle = 0;
    while !commands_queue.is_empty() {
        cycle += 1;
        if (register - (cycle - 1) % 40).abs() < 2 {
            crt[(cycle - 1) as usize] = '#';
        }

        if skip_cycle {
            skip_cycle = false;
        } else {
            match commands_queue.pop().unwrap() {
                Cmd::Add(v) => {
                    to_add = Some(v);
                    skip_cycle = true;
                }
                Cmd::Nop => {}
            }
            continue;
        }

        to_add.map_or((), |v| register += v);
    }
    render_crt(&crt);
}

#[cfg(test)]
mod test {
    use crate::d10::p1;
    use crate::d10::p2;

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d10_test.txt"), 13140);
    }

    #[test]
    fn p2_works() {
        p2("input/d10_test.txt");
    }

    #[test]
    fn p1_task() {
        assert_eq!(p1("input/d10.txt"), 12460);
    }

    #[test]
    fn p2_task() {
        p2("input/d10.txt");
    }
}
