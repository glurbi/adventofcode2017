use std::fs::File;
use std::io::prelude::*;

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
            raw
        }
    }
}

pub fn day19() {
    part1();
    //part2();
}

fn part1() {
    println!("Part 1");
    let diagram = Diagram::from_file("input/Day19-test.txt");
    println!("{:?}", diagram);
}

fn part2() {
    println!("Part 2");
}

