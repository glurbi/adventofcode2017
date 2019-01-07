const INPUT: &'static str = include_str!("../../input/Day4.txt");

fn main() {
    day1();
}

fn day1() {
    run1();
    run2();
}

fn run1() {
    let mut count = 0;
    for password in INPUT.lines() {
        if is_valid(password) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn is_valid(password: &str) -> bool {
    let mut v: Vec<&str> = password.split(' ').collect();
    let len_orig = v.len();
    v.sort();
    v.dedup();
    len_orig == v.len()
}

fn run2() {
    let mut count = 0;
    for password in INPUT.lines() {
        if is_valid_2(password) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn is_valid_2(password: &str) -> bool {
    let v: Vec<&str> = password.split(' ').collect();
    let len_orig = v.len();
    let mut v: Vec<String> = v.iter().map(|s| {
        let mut w = s.chars().collect::<Vec<char>>();
        w.sort();
        w.iter().collect()
    }).collect();
    v.sort();
    v.dedup();
    len_orig == v.len()
}
