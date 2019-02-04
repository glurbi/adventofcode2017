use std::fmt;
use std::str;
use std::ops::{Index, IndexMut};

const INPUT: &'static str = include_str!("../../input/Day22.txt");

#[derive(Debug)]
enum Dir { N, S, E, W }
use Dir::{ N, S, E, W };

struct Cluster {
    width: i64,
    height: i64,
    v: Vec<u8>,
    vx: i64,
    vy: i64,
    dir: Dir,
    infections: usize,
}

impl Cluster {
    fn increase_capacity(&mut self) {
        let new_width = self.width*3;
        let new_height = self.height*3;
        let mut new_v = vec!['.' as u8; (new_width * new_height) as usize];

        for i in 0..self.v.len() {
            let x = i as i64 % self.width;
            let y = i as i64 / self.width;
            let new_x = self.width + x;
            let new_y = self.height + y;
            let new_p = (new_x + new_y * new_width) as usize;
            let p = (x + y * self.width) as usize;
            new_v[new_p] = self.v[p];
        }

        self.vx = self.vx + self.width;
        self.vy = self.vy + self.height;
        self.width = new_width;
        self.height = new_height;
        self.v = new_v;
        println!("vx={}, vy={}, dir={:?}, infections:{}", self.vx, self.vy, self.dir, self.infections);
    }

    fn is_infected(&self) -> bool {
        self[(self.vx, self.vy)] as char == '#'
    }

    fn step(&mut self) {
        let vx = self.vx;
        let vy = self.vy;
        if self.is_infected() {
            self.dir = match self.dir {
                N => W,
                W => S,
                S => E,
                E => N,
            };
            self[(vx, vy)] = '.' as u8;
        } else {
            self.dir = match self.dir {
                N => E,
                E => S,
                S => W,
                W => N,
            };
            self[(vx, vy)] = '#' as u8;
            self.infections += 1;
        }
        match self.dir {
            N => self.vy -= 1,
            E => self.vx -= 1,
            S => self.vy += 1,
            W => self.vx += 1,
        };
        if self.vx < 0 || self.vy < 0 || self.vx >= self.width || self.vy >= self.height {
            self.increase_capacity();
        }        
    }
}

impl fmt::Debug for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        for i in 0..self.height {
            let from = (i*self.width) as usize;
            let to = ((i+1)*self.width) as usize;
            writeln!(f, "{}", str::from_utf8(&self.v[from..to]).unwrap())?;
        };
        writeln!(f, "vx={}, vy={}, dir={:?}, infections:{}", self.vx, self.vy, self.dir, self.infections)?;
        Ok(())
    }
}

impl Index<(i64,i64)> for Cluster {
    type Output = u8;

    fn index(&self, (x,y): (i64,i64)) -> &u8 {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            &('.' as u8)            
        } else {
            let p = (x + (y*self.width)) as usize;
            &self.v[p]
        }
    }
}

impl IndexMut<(i64,i64)> for Cluster {

    fn index_mut(&mut self, (x,y): (i64,i64)) -> &mut u8 {
        let p = (x + (y*self.width)) as usize;
        &mut self.v[p]
    }
}

fn main() {
    day1();
    day2();
}

fn day1() {
    let mut test_cluster = Cluster {
        width: 3,
        height: 3,
        v: "..##.....".as_bytes().iter().cloned().collect(),
        vx: 1,
        vy: 1,
        dir: N,
        infections: 0,
    };
    println!("{:?}", test_cluster);
    for _ in 0..70 {
        test_cluster.step();
        //println!("{:?}", test_cluster);
    }
    println!("{:?}", test_cluster);

    let mut actual_cluster = Cluster {
        width: 25,
        height: 25,
        v: INPUT.chars().filter(|c| !c.is_ascii_whitespace()).map(|c| c as u8).collect(),
        vx: 12,
        vy: 12,
        dir: N,
        infections: 0,
    };
    println!("{:?}", actual_cluster);
    for _ in 0..10000 {
        actual_cluster.step();
        //println!("{:?}", test_cluster);
    }
    println!("{}", actual_cluster.infections);
}

fn day2() {
    let mut test_cluster = Cluster {
        width: 3,
        height: 3,
        v: "..##.....".as_bytes().iter().cloned().collect(),
        vx: 1,
        vy: 1,
        dir: N,
        infections: 0,
    };
    println!("{:?}", test_cluster);
    for i in 0..100000 {
        test_cluster.step();
        if i % 10000 == 0 {
            print!(".")
        }
        //println!("{:?}", test_cluster);
    }
    println!("{:?}", test_cluster);
}
