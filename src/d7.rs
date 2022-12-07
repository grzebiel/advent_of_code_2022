use itertools;
use std::collections::HashMap;
use std::fs;
use std::str::SplitWhitespace;

fn parse_command(cmd: &mut SplitWhitespace, cwd: &mut Vec<String>) {
    match cmd.next().unwrap() {
        "cd" => match cmd.next().unwrap() {
            "/" => {
                cwd.clear();
                cwd.push("".to_string());
            }
            ".." => {
                cwd.pop();
            }
            s => {
                cwd.push(String::from(s));
            }
        },
        "ls" => {}
        _ => panic!("unexpect path!"),
    }
}

fn make_path(cwd: &Vec<String>, filename: &str) -> String {
    let mut cwd_string = cwd.join("/");
    cwd_string.push_str("/");
    cwd_string.push_str(filename);
    cwd_string
}

fn parse_file(words: &mut SplitWhitespace, cwd: &Vec<String>, size_str: &str) -> (String, i64) {
    let size = size_str.parse::<i64>().unwrap();
    let path = make_path(&cwd, words.next().unwrap());
    (path, size)
}

fn get_dirs(path: &String) -> Vec<String> {
    let mut elements: Vec<String> = path
        .split("/")
        // .filter(|s| *s == "")
        .map(String::from)
        .collect();
    elements.pop(); // filename
    let mut path_work = String::new();
    let mut ret = vec![];
    for el in elements {
        if path_work.chars().last() != Some('/') {
            path_work.push_str("/");
        }
        path_work.push_str(&el);
        ret.push(path_work.clone());
    }
    ret
}

fn sum_directories(fs: &HashMap<String, i64>) -> HashMap<String, i64> {
    let files = fs.clone();
    let mut directories = HashMap::<String, i64>::new();
    for (path, filesize) in files {
        for dir in get_dirs(&path) {
            directories
                .entry(dir)
                .and_modify(|e| *e += filesize)
                .or_insert(filesize);
        }
    }
    directories
}

pub fn parse(input: String) -> (HashMap<String, i64>, HashMap<String, i64>) {
    let mut files: HashMap<String, i64> = HashMap::new();

    let mut cwd = Vec::new();
    for line in input.split("\n") {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("$") => parse_command(&mut words, &mut cwd),
            Some("dir") => {}
            Some(size_str) => {
                let (path, size) = parse_file(&mut words, &cwd, size_str);
                files.insert(path, size);
            }
            None => {}
        }
    }

    (sum_directories(&files), files)
}

pub fn p1() -> i64 {
    let input = fs::read_to_string("input/d7.txt").unwrap();
    let (directories, _) = parse(input);
    directories
        .into_values()
        .filter(|size| size < &100000)
        .sum()
}

pub fn p2() -> i64 {
    let input = fs::read_to_string("input/d7.txt").unwrap();
    let (directories, _) = parse(input);
    let available = 70000000 - directories["/"];
    let needed = 30000000;
    let missing = needed - available;
    directories
        .into_values()
        .filter(|size| *size >= missing)
        .min()
        .unwrap()
}
