use std::fs::File;
use std::io::prelude::*;

fn main() {
    day13();
}

fn day13() {
    day13_1();
    day13_2();
}

#[derive(Debug, Clone, Copy)]
struct Layer {
    scanner_pos: usize,
    range_mod: usize,
    range_orig: usize,
}

#[derive(Debug, Clone)]
struct Firewall {
    layers: Vec<Option<Layer>>,
    packet_depth: usize,
}

impl Firewall {
    fn new() -> Firewall {
        Firewall {
            layers: Vec::new(),
            packet_depth: 0,
        }
    }

    fn load(input: &str) -> Firewall {
        let mut firewall = Firewall::new();
        for line in input.lines() {
            let mut it = line.split(": ");
            let depth = it.next().unwrap().parse::<usize>().unwrap();
            let range_orig = it.next().unwrap().parse::<usize>().unwrap(); 
            let range_mod = range_orig * 2 - 2;
            firewall.layers.resize(depth, None);
            let layer = Layer {
                scanner_pos: 0,
                range_orig,
                range_mod,
            };
            firewall.layers.insert(depth, Some(layer));
        }
        firewall
    }

    fn local_severity(&self) -> usize {
        match self.layers[self.packet_depth] {
            Some(l) => (l.scanner_pos == 0) as usize * self.packet_depth * l.range_orig,
            None => 0
        }
    }

    fn is_caught(&self, depth: usize, delay: usize) -> bool {
        //println!("delay = {}, depth = {}", delay, depth);
        match self.layers[depth] {
            Some(l) => (delay+depth) % l.range_mod == 0,
            None => false,
        }
    }

    fn total_severity(&mut self) -> usize {
        let mut severity = 0;
        self.move_scanner(1);
        for _ in 0..self.layers.len()-1 {
            //println!("{:?}", self);
            //println!("{}", severity);
            self.move_packet();
            severity += self.local_severity();
            self.move_scanner(1);
        }
        severity
    }

    fn until_not_caught2(&self) {
        let mut delay = 0;
        loop {
            let caught = (0..self.layers.len()).any(|depth| self.is_caught(depth, delay));
            if !caught {
                println!("delay = {}, caught = {}", delay, caught);
                break;
            }
            delay += 1;
        }
    }

    fn move_packet(&mut self) {
        self.packet_depth += 1;
    }

    fn move_scanner(&mut self, amount: usize) {
        for o in self.layers.iter_mut() {
            *o = match o {
                Some(l) => Some(Layer {
                     scanner_pos: (l.scanner_pos + amount) % l.range_mod,
                     range_orig: l.range_orig,
                     range_mod: l.range_mod,
                }),
                None => None,
            };
        }
    }
}

fn day13_1() {
    let input = read_to_string("input/Day13-test.txt");
    println!("{}", &input);
    let mut firewall = Firewall::load(&input);
    let severity = firewall.total_severity();
    println!("severity = {}", severity);

    let input = read_to_string("input/Day13.txt");
    let mut firewall = Firewall::load(&input);
    let severity = firewall.total_severity();
    println!("severity = {}", severity);
}

fn day13_2() {
    let input = read_to_string("input/Day13-test.txt");
    let firewall = Firewall::load(&input);
    Firewall::until_not_caught2(&firewall);

    let input = read_to_string("input/Day13.txt");
    let firewall = Firewall::load(&input);
    Firewall::until_not_caught2(&firewall);
}

fn read_to_string(file_name: &str) -> String {
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut content = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut content).expect(&err_read);
    content
}