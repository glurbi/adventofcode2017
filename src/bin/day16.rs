use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    day16();
}

fn day16() {
    part1();
    part2();
}

fn part1() {

    println!("Part 1");

    let progs = String::from("abcde");
    let raw_moves = String::from("s1,x3/4,pe/b");
    let mut d16 = Part1::new(progs, raw_moves);
    d16.dance();
    println!("{:?}", d16.progs);

    let progs = String::from("abcdefghijklmnop");
    let raw_moves = read_to_string("input/Day16.txt");
    let mut d16 = Part1::new(progs, raw_moves);
    d16.dance();
    println!("{:?}", d16.progs);
}

fn read_to_string(file_name: &str) -> String {
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut content = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut content).expect(&err_read);
    content
}

#[derive(Debug)]
struct Part1 {
    progs: String,
    raw_moves: String,
}

impl Part1 {

    fn new(progs: String, raw_moves: String) -> Part1 {
        Part1 {
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
                    self.progs = Part1::spin(&self.progs, count);
                },
                'x' => {
                    let mut indexes = a_move[1..].split("/");
                    let i1 = indexes.next().unwrap().parse::<usize>().unwrap();
                    let i2 = indexes.next().unwrap().parse::<usize>().unwrap();
                    self.progs = Part1::exchange(&self.progs, i1, i2);
                },
                'p' => {
                    let mut swappers = a_move[1..].split("/");
                    let s1 = swappers.next().unwrap().chars().nth(0).unwrap();
                    let s2 = swappers.next().unwrap().chars().nth(0).unwrap();
                    self.progs = Part1::partner(&self.progs, s1, s2);
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
        Part1::exchange(&progs, i1, i2)
    }

}

fn part2() {

    println!("Part 2");

    //let raw_progs = String::from("abcde");
    //let raw_moves = String::from("s1,x3/4,pe/b");

    let raw_progs = String::from("abcdefghijklmnop");
    let raw_moves = read_to_string("input/Day16.txt");
    let mut part2 = Part2::new(&raw_progs, &raw_moves);
    println!("{:?}", part2.progs);
    let mut progset: HashSet<String> = HashSet::new();
    let mut progvec: Vec<String> = Vec::new();
    let mut count = 0;
    progset.insert(part2.progs_as_string());
    for c in 0..200 {
        println!("{:?} -> {:?}", c, part2.progs_as_string());
        part2.dance_one_round();
        progvec.push(part2.progs_as_string());
        if !progset.insert(part2.progs_as_string()) {
            count = c + 1;
            println!("count = {:?}", count);
            break;
        }
    }
    let i = 1000000000 % count;
    println!("{:?}", i);
}

#[derive(Debug)]
enum Move {
    Spin { n: usize },
    Exchange { i1: usize, i2: usize },
    Partner { p1: char, p2: char },
}

use Move::{Spin,Exchange,Partner};

#[derive(Debug)]
struct Part2 {
    progs: Vec<char>,
    moves: Vec<Move>,
}

impl Part2 {

    fn new(raw_progs: &str, raw_moves: &str) -> Part2 {
        Part2 {
            progs: raw_progs.chars().collect(),
            moves: Part2::extract(raw_moves),
        }
    }

    fn extract(raw_moves: &str) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let moves_coll: Vec<&str> = raw_moves.split(",").collect();
        for a_move in moves_coll {
            //println!("{}", a_move);
            let c = a_move.chars().nth(0).unwrap();
            match c {
                's' => {
                    let count = a_move[1..].parse::<usize>().unwrap();
                    result.push(Spin { n: count });
                },
                'x' => {
                    let mut indexes = a_move[1..].split("/");
                    let i1 = indexes.next().unwrap().parse::<usize>().unwrap();
                    let i2 = indexes.next().unwrap().parse::<usize>().unwrap();
                    result.push(Exchange { i1, i2 });
                },
                'p' => {
                    let mut swappers = a_move[1..].split("/");
                    let p1 = swappers.next().unwrap().chars().nth(0).unwrap();
                    let p2 = swappers.next().unwrap().chars().nth(0).unwrap();
                    result.push(Partner { p1, p2 });
                }
                _ => panic!("...")
            }
            //println!("{:?}", self);
        }
        result
    }

    fn dance_one_round(&mut self) {
        for a_move in &self.moves {
            match *a_move {
                Spin { n } => Part2::spin(&mut self.progs, n),
                Exchange { i1, i2 } => Part2::exchange(&mut self.progs, i1, i2),
                Partner { p1, p2} => Part2::partner(&mut self.progs, p1, p2),
            }
        }
    }

    fn progs_as_string(&mut self) -> String {
        self.progs.iter().collect()
    }

    fn spin(progs: &mut Vec<char>, n: usize) {
        let mut result: Vec<char> = Vec::with_capacity(progs.len());
        result.extend_from_slice(&progs[progs.len()-n..]);
        result.extend_from_slice(&progs[..progs.len()-n]);
        progs.clear();
        progs.extend_from_slice(&result);
    }

    fn exchange(progs: &mut Vec<char>, i1: usize, i2: usize) {
        let tmp = progs[i1];
        progs[i1] = progs[i2];
        progs[i2] = tmp;
    }

    fn partner(progs: &mut Vec<char>, p1: char, p2: char) {
        let i1 = progs.iter().position(|c| *c == p1).unwrap();
        let i2 = progs.iter().position(|c| *c == p2).unwrap();
        Part2::exchange(progs, i1, i2);
    }

}


