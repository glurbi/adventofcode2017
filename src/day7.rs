use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Prog {
    name: String,
    weight: i32,
    children: Vec<String>,
}

pub fn day7() {
    let mut progs: HashMap<String,Prog> = HashMap::new();
    let mut relations: HashMap<String,String> = HashMap::new();
    let file = File::open("input/day7.txt").expect("Failed to open day7.txt");
    let re = Regex::new(r"([a-z]+) \((\d+)\)(?: -> )?(.+)?").unwrap();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if !re.is_match(&line) {
            println!("{}", &line);
            continue;
        }
        let captures = match re.captures(&line) {
            Some(captures) => captures,
            None => continue,
        };
        let name = captures.get(1).unwrap().as_str().to_string();
        let weight = captures.get(2).unwrap().as_str().to_string().parse::<i32>().unwrap();
        let mut children: Vec<String> = Vec::new();
        if let Some(sub_names) = captures.get(3) {
            for sub_name in sub_names.as_str().to_string().split(", ") {
                relations.insert(sub_name.to_string(), name.to_string());
                children.push(sub_name.to_string());
            }
        }

        let prog = Prog { name: name.to_string(), weight, children };
        println!("{:?}", captures);
        println!("{:?}", prog);
        progs.insert(name.to_string(), prog);
    }

    let mut opt = Some(relations.iter().next().unwrap().0);
    let mut bottom = "nil".to_string();
    loop {
        println!("{:?}", opt);
        match opt {
            Some(name) => {
                opt = relations.get(name);
                bottom = name.to_string();
            },
            None => break,
        } 
    }
    println!("{:?}", bottom);
}
