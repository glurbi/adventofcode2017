extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    day7();
}

#[derive(Debug, Clone)]
struct Prog {
    name: String,
    weight: i32,
    children: Vec<String>,
}

fn day7() {
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
        //println!("{:?}", captures);
        //println!("{:?}", prog);
        progs.insert(name.to_string(), prog);
    }

    let mut opt = Some(relations.iter().next().unwrap().0);
    let mut bottom = "nil".to_string();
    loop {
        //println!("{:?}", opt);
        match opt {
            Some(name) => {
                opt = relations.get(name);
                bottom = name.to_string();
            },
            None => break,
        } 
    }
    println!("{:?}", bottom);

    let mut refs: Vec<&Prog> = Vec::new();
    let mut idx: Vec<usize> = Vec::new();
    refs.push(progs.get(&bottom).unwrap());
    idx.push(0);
    //println!("{:?}", refs);
    while !refs.is_empty() {
        let prog = refs.last().unwrap().clone();
        let i = *idx.last().unwrap();
        if prog.children.len() == i {
            if i > 0 {
                let child_progs = prog.children.iter().map(|child| progs.get(&child.to_string()).unwrap()).collect();
                let tower_weights = tower_weights(&child_progs, &progs);
                let all_same = tower_weights.iter().all(|&w| w == tower_weights[0]);
                if !all_same {
                    let child_weights: Vec<i32> = child_progs.iter().map(|prog| prog.weight).collect();
                    println!("{:?},{:?},{:?},{}",prog,tower_weights,child_weights,all_same);
                    break;
                }
            }
            refs.pop();
            idx.pop();
        } else {
            let new_i = idx.pop().unwrap() + 1;
            idx.push(new_i);
            refs.push(progs.get(&prog.children[i]).unwrap());
            idx.push(0);
        }
    }
}

fn tower_weights(roots: &Vec<&Prog>, progs: &HashMap<String,Prog>) -> Vec<i32> {
    roots.iter().map(|&root| tower_weight(root, &progs)).collect()
}

fn tower_weight(root: &Prog, progs: &HashMap<String,Prog>) -> i32 {
    let mut refs: Vec<&Prog> = Vec::new();
    let mut idx: Vec<usize> = Vec::new();
    refs.push(&root);
    idx.push(0);
    let mut sum = 0;
    let mut acc: Vec<i32> = Vec::new();
    while !refs.is_empty() {
        let prog = refs.last().unwrap().clone();
        let i = *idx.last().unwrap();
        if prog.children.len() == i {
            acc.push(prog.weight);
            sum += prog.weight;
            refs.pop();
            idx.pop();
        } else {
            let new_i = idx.pop().unwrap() + 1;
            idx.push(new_i);
            refs.push(progs.get(&prog.children[i]).unwrap());
            idx.push(0);
        }
    }
    sum
}