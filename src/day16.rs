use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Day16 {
    progs: String,
    raw_moves: String,
}

impl Day16 {

    fn new(progs: String, raw_moves: String) -> Day16 {
        Day16 {
            progs,
            raw_moves,
        }
    }

    fn dance(&mut self) {
        //println!("{:?}", self);
        let moves: Vec<&str> = self.raw_moves.split(",").collect();
        for a_move in moves {
            //println!("{}", a_move);
            let c = a_move.chars().nth(0).unwrap();
            match c {
                's' => {
                    let count = a_move[1..].parse::<usize>().unwrap();
                    self.progs = Day16::spin(&self.progs, count);
                },
                'x' => {
                    let mut indexes = a_move[1..].split("/");
                    let i1 = indexes.next().unwrap().parse::<usize>().unwrap();
                    let i2 = indexes.next().unwrap().parse::<usize>().unwrap();
                    self.progs = Day16::exchange(&self.progs, i1, i2);
                },
                'p' => {
                    let mut swappers = a_move[1..].split("/");
                    let s1 = swappers.next().unwrap().chars().nth(0).unwrap();
                    let s2 = swappers.next().unwrap().chars().nth(0).unwrap();
                    self.progs = Day16::partner(&self.progs, s1, s2);
                }
                _ => panic!("...")
            }
            //println!("{:?}", self);
        }
    }

    fn spin(progs: &str, n: usize) -> String {
        let mut output: String = String::from("");
        output.push_str(&progs[progs.len()-n..]);
        output.push_str(&progs[..progs.len()-n]);
        output
    }

    fn exchange(progs: &str, i1: usize, i2: usize) -> String {
        let mut v: Vec<char> = progs.chars().collect();
        let tmp = v[i1];
        v[i1] = v[i2];
        v[i2] = tmp;
        let output: String = v.into_iter().collect();
        output
    }

    fn partner(progs: &str, p1: char, p2: char) -> String {
        let v: Vec<char> = progs.chars().collect();
        let i1 = v.iter().position(|c| *c == p1).unwrap();
        let i2 = v.iter().position(|c| *c == p2).unwrap();
        Day16::exchange(&progs, i1, i2)
    }

}

pub fn day16() {
    day16_1();
    day16_2();
}

fn day16_1() {
    println!("Part 1");

    let progs = String::from("abcde");
    let raw_moves = String::from("s1,x3/4,pe/b");
    let mut d16 = Day16::new(progs, raw_moves);
    d16.dance();
    println!("{:?}", d16);

    let progs = String::from("abcdefghijklmnop");
    let raw_moves = read_to_string("input/Day16.txt");
    let mut d16 = Day16::new(progs, raw_moves);
    d16.dance();
    println!("{:?}", d16);
}

fn day16_2() {
    println!("Part 2");
}

fn read_to_string(file_name: &str) -> String {
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut content = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut content).expect(&err_read);
    content
}


