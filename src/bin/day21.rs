use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str;

const INPUT: &'static str = include_str!("../../input/Day21.txt");

#[derive(Hash, Eq, PartialEq, Clone)]
struct Image {
    content: Vec<u8>,
    size: usize,
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<Image,Image>,
}

impl Image {

    fn init() -> Image {
        Image {
            content: ".#...####".as_bytes().iter().cloned().collect(),
            size: 3,
        }
    }

    fn from_text(input: &str) -> Image {
        let size = input.find('/').unwrap();
        let content = input.chars().filter(|&c| c != '/').map(|c| c as u8).collect();
        Image {
            content,
            size,
        }
    }

    fn set(&mut self, i: usize, j: usize, c: u8) {
        self.content[i+j*self.size] = c;
    }

    fn get(&self, i: usize, j: usize) -> u8 {
        self.content[i+j*self.size]
    }

    fn new(size: usize) -> Image {
        let content = " ".repeat(size*size).chars().map(|c| c as u8).collect();
        Image {
            content,
            size,
        }
    }

    fn equivalents(img: &Image) -> HashSet<Image> {
        let mut hs: HashSet<Image> = HashSet::new();
        hs.insert(img.clone());

        // rotate
        let img_rot = img.rotate_left();
        hs.insert(img_rot.clone());
        let img_rot = img_rot.rotate_left();
        hs.insert(img_rot.clone());
        let img_rot = img_rot.rotate_left();
        hs.insert(img_rot.clone());

        // horizontal symetry
        let mut img1 = Image::new(img.size);
        for j in 0..img.size {
            for i in 0..img.size {
                let new_j = img.size - j - 1;
                let c = img.get(i, j);
                img1.set(i, new_j, c);
            }
        }
        hs.insert(img1.clone());

        // rotate
        let img_rot = img1.rotate_left();
        hs.insert(img_rot.clone());
        let img_rot = img_rot.rotate_left();
        hs.insert(img_rot.clone());
        let img_rot = img_rot.rotate_left();
        hs.insert(img_rot.clone());

        hs
    }

    fn rotate_left(&self) -> Image {
        let mut res = self.clone();
        if res.size == 3 {
            res.set(0, 0, self.get(2, 0));
            res.set(1, 0, self.get(2, 1));
            res.set(2, 0, self.get(2, 2));
            res.set(0, 1, self.get(1, 0));
            res.set(1, 1, self.get(1, 1));
            res.set(2, 1, self.get(1, 2));
            res.set(0, 2, self.get(0, 0));
            res.set(1, 2, self.get(0, 1));
            res.set(2, 2, self.get(0, 2));
        } else {
            res.set(0, 0, self.get(1, 0));
            res.set(1, 0, self.get(1, 1));
            res.set(1, 1, self.get(0, 1));
            res.set(0, 1, self.get(0, 0));
        }
        res
    }

    fn get_subimage(&self, i: usize, j: usize, size: usize) -> Image {
        let mut res = Image::new(size);
        for ii in i..i+size {
            for jj in j..j+size {
                res.set(ii-i, jj-j, self.get(ii, jj));
            }
        }
        res
    }

    fn set_subimage(&mut self, i: usize, j: usize, sub: &Image) {
        for ii in i..i+sub.size {
            for jj in j..j+sub.size {
                self.set(ii, jj, sub.get(ii-i, jj-j));
            }
        }
    }

    fn step(&mut self, rules: &Rules) -> Image {
        let block_count = if self.size % 2 == 0 { self.size / 2 } else { self.size / 3 };
        let old_block_size = if self.size % 2 == 0 { 2 } else { 3 };
        let new_block_size = if self.size % 2 == 0 { 3 } else { 4 };
        let new_size = if self.size % 2 == 0 { (self.size/2)*3 } else { (self.size/3)*4 };
        let mut res = Image::new(new_size);

        for bj in 0..block_count {
            for bi in 0..block_count {
                let sub = self.get_subimage(bi*old_block_size, bj*old_block_size, old_block_size);
                //println!("{:?}", sub);
                let sub = &rules.rules[&sub];
                res.set_subimage(bi*new_block_size, bj*new_block_size, sub);
            }
        }

        res
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        for i in 0..self.size {
            writeln!(f, "{}", str::from_utf8(&self.content[i*self.size..(i+1)*self.size]).unwrap())?;
        }
        Ok(())
    }
}

impl Rules {

    fn new() -> Rules {
        Rules {
            rules: HashMap::new(),
        }
    }

    fn from_text(input: &str) -> Rules {
        let mut rules = Rules::new();
        for rule in input.lines() {
            rules.insert(rule.trim());
        }
        rules
    }

    fn insert(&mut self, rule: &str) {
        let in_image = Image::from_text(rule.split(" => ").nth(0).unwrap());
        let in_equivs = Image::equivalents(&in_image);
        let out_image = Image::from_text(rule.split(" => ").nth(1).unwrap());

        for img in in_equivs.iter() {
            self.rules.insert(img.clone(), out_image.clone());
        }
    }
}

fn main() {
    day1();
    day2();
}

fn day1() {
    let mut image = Image::init();
    let rules = Rules::from_text(INPUT);
    //let mut rules = Rules::new();
    //rules.insert("../.# => ##./#../...");
    //rules.insert(".#./..#/### => #..#/..../..../#..#");
    //println!("{:?}", rules);

    //println!("{:?}", Image::equivalents(&image));
    //println!("{:?}", image);
    //println!("{:?}", image.rotate_left());
    println!("{:?}", image);
    image = image.step(&rules);
    println!("{:?}", image);
    image = image.step(&rules);
    println!("{:?}", image);
    image = image.step(&rules);
    println!("{:?}", image);
    image = image.step(&rules);
    println!("{:?}", image);
    image = image.step(&rules);
    println!("{:?}", image);

    let s: Vec<u8> = image.content.iter().filter(|&c| *c == '#' as u8).cloned().collect();
    println!("{}", s.len());
}

fn day2() {
    let mut image = Image::init();
    let rules = Rules::from_text(INPUT);

    println!("{:?}", image);

    for _ in 0..18 {
        image = image.step(&rules);
        println!("{:?}", image);
    }

    let s: Vec<u8> = image.content.iter().filter(|&c| *c == '#' as u8).cloned().collect();
    println!("{}", s.len());
}
