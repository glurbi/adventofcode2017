use std::fs::File;
use std::io::prelude::*;

use day19::Direction::{ N, E, S, W };

#[derive(Debug, Clone, Copy)]
enum Direction { N, E, S, W }

fn other_dirs(dir: Direction) -> Vec<Direction> {
    match dir {
        N => vec!{ E, W }, 
        S => vec!{ E, W },
        E => vec!{ N, S },
        W => vec!{ N, S },
    }
}

#[derive(Debug)]
struct Diagram {
    raw: String,
}

impl Diagram {

    fn from_file(file_name: &str) -> Diagram {
        let err_open = format!("Failed to open {}", &file_name);
        let mut file = File::open(file_name).expect(&err_open);
        let mut raw = String::new();
        let err_read = format!("Failed to read {}", &file_name);
        file.read_to_string(&mut raw).expect(&err_read);
        Diagram {
            raw,
        }
    }

    fn char_at(&self, x: usize, y: usize) -> char {
        self.raw.lines().nth(y).unwrap().chars().nth(x).unwrap()
    }

    fn char_at_dir(&self, x: usize, y: usize, dir: Direction) -> (usize, usize, char) {
        match dir {
            N => (x, y - 1, self.char_at(x, y - 1)),
            S => (x, y + 1, self.char_at(x, y + 1)),
            E => (x + 1, y, self.char_at(x + 1, y)),
            W => (x - 1, y, self.char_at(x - 1, y)),
        }
    }

    fn next_char(&self, x: usize, y: usize, dir: Direction) -> (usize, usize, char, Direction) {
        let new_dir = match self.char_at(x, y) {
            '+' => {
                let dirs = other_dirs(dir);
                match self.char_at_dir(x, y, dirs[0]) {
                    (_, _, ' ') => dirs[1],
                    _ => dirs[0],
                }
            },
            _ => dir,
        };
        let (x, y, c) = self.char_at_dir(x, y, new_dir);
        (x, y, c, new_dir)
    }
}

#[derive(Debug)]
struct Packet {
    x: usize,
    y: usize,
    c: char,
    steps: usize,
    letters: String,
    dir: Direction,
}

impl Packet {

    fn from_diagram(diagram: &Diagram) -> Packet {
        let x = diagram.raw.lines().next().unwrap().find('|').unwrap();
        let c = diagram.raw.lines().nth(0).unwrap().chars().nth(x).unwrap();
        Packet {
            x,
            y: 0,
            c,
            steps: 0,
            letters: String::new(),
            dir: S,
        }
    }

    fn step(&mut self, diagram: &Diagram) {
        let (x, y, c, d) = diagram.next_char(self.x, self.y, self.dir);
        self.x = x;
        self.y = y;
        self.c = c;
        self.dir = d;
        self.steps += 1;
        let non_letters = " |-+";
        if !non_letters.contains(c) {
            self.letters.push(c);
        }
    }

    fn run(&mut self, diagram: &Diagram) -> String {
        while self.c != ' ' {
            //println!("{:?}", packet);
            self.step(&diagram);
        }
        self.letters.clone()
    }
}

pub fn day19() {
    part1();
    //part2();
}

fn part1() {
    println!("Part 1");

    let diagram = Diagram::from_file("input/Day19-test.txt");
    let mut packet  = Packet::from_diagram(&diagram);
    packet.run(&diagram);
    println!("{:?}", packet);

    let diagram = Diagram::from_file("input/Day19.txt");
    let mut packet  = Packet::from_diagram(&diagram);
    packet.run(&diagram);
    println!("{:?}", packet);
}

fn part2() {
    println!("Part 2");
}

