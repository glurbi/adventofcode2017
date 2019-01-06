use std::collections::HashMap;
use std::ops::{Index, IndexMut};

fn main() {
    day1();
}

fn day1() {
    run1();
    run2();
}

fn run1() {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    let mut pos = 1;
    const GOAL: i32 = 265149;

    while pos < GOAL {

        while pos < GOAL && x <= max_x {
            x += 1;
            pos += 1;
        }
        max_x = x;

        while pos < GOAL && y <= max_y {
            y += 1;
            pos += 1;
        }
        max_y = y;

        while pos < GOAL && x >= min_x {
            x-= 1;
            pos += 1;
        }
        min_x = x;

        while pos < GOAL && y >= min_y {
            y-= 1;
            pos += 1;
        }
        min_y = y;
    }

    println!("{:?} {}", (x, y), x.abs() + y.abs());
}

struct Memory {
    dic: HashMap<(i32, i32), i32>,
}

impl Memory {

    fn new() -> Memory {   
        let mut mem = Memory {
            dic: HashMap::new(),
        };
        mem[(0,0)] = 1;
        mem
    }

    fn compute(&self, x: i32, y: i32) -> i32 {
        return self[(x-1, y+1)] + self[(x-0, y+1)] + self[(x+1, y+1)]
             + self[(x-1, y-0)] +                    self[(x+1, y+0)]
             + self[(x-1, y-1)] + self[(x-0, y-1)] + self[(x+1, y-1)];
    }

}

impl Index<(i32, i32)> for Memory {
    type Output = i32;
    fn index(&self, idx: (i32, i32)) -> &i32 {
        if self.dic.contains_key(&idx) {
            &self.dic[&idx]
        } else {
            &0
        }
    }
}

impl IndexMut<(i32, i32)> for Memory { 
    fn index_mut(&mut self, idx: (i32, i32)) -> &mut i32 {
        self.dic.entry(idx).or_insert(0)
    }
}

impl Memory {
    fn values(&mut self) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);

        let (mut x, mut y): (i32, i32) = (0, 0);
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);

        for _ in 0..5 {
            while x <= max_x {
                x += 1;
                self[(x, y)] = self.compute(x, y);
                v.push(self[(x, y)]);
            }
            max_x = x;

            while y <= max_y {
                y += 1;
                self[(x, y)] = self.compute(x, y);
                v.push(self[(x, y)]);
            }
            max_y = y;

            while x >= min_x {
                x-= 1;
                self[(x, y)] = self.compute(x, y);
                v.push(self[(x, y)]);
            }
            min_x = x;

            while y >= min_y {
                y-= 1;
                self[(x, y)] = self.compute(x, y);
                v.push(self[(x, y)]);
            }
            min_y = y;
        }

        v
    }
}

fn run2() {
    let mut mem = Memory::new();
    for i in mem.values() {
        println!("{}", i);
        if i > 265149 {
            break;
        }
    }
}
