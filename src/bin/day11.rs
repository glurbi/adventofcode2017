use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    day11();
}

pub fn day11() {
    day11_1();
    day11_2();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

use Direction::*;

fn day11_1() {
    println!("{}", distance("ne,ne,ne"));
    println!("{}", distance("ne,ne,sw,sw"));
    println!("{}", distance("ne,ne,s,s"));
    println!("{}", distance("se,sw,se,sw,sw"));

    let file_name = "input/day11.txt";
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut input = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut input).expect(&err_read);
    println!("{}", distance(&input));
}

fn distance(input: &str) -> i32 {
    if input.len() > 100 {
        println!("{}...", &input[..100]);
    } else {
        println!("{}", input);
    }
    let res = load(&input);
    //println!("{:?}", res);
    let res = split(&res);
    println!("{:?}", res);
    let res = simplify(&res);
    println!("{:?}", res);
    res.values().sum()
}

fn simplify(dirs: &HashMap<Direction, i32>) -> HashMap<Direction, i32> {
    let mut res = HashMap::new();
    let mut n = dirs[&N];
    let mut ne = dirs[&NE];
    let mut se = dirs[&SE];
    let mut s = dirs[&S];
    let mut sw = dirs[&SW];
    let mut nw = dirs[&NW];

    for _ in 0..2 {

        let min_nw_ne = cmp::min(nw, ne);
        nw -= min_nw_ne;
        ne -= min_nw_ne;
        n += min_nw_ne;

        let min_n_se = cmp::min(n, se);
        n -= min_n_se;
        se -= min_n_se;
        ne += min_n_se;

        let min_ne_s = cmp::min(ne, s);
        ne -= min_ne_s;
        s -= min_ne_s;
        se += min_ne_s;

        let min_sw_se = cmp::min(sw, se);
        sw -= min_sw_se;
        se -= min_sw_se;
        s += min_sw_se;

        let min_s_nw = cmp::min(s, nw);
        s -= min_s_nw;
        nw -= min_s_nw;
        sw += min_s_nw;

        let min_sw_n = cmp::min(sw, n);
        sw -= min_sw_n;
        n -= min_sw_n;
        nw += min_sw_n;

        let min_n_s = cmp::min(n, s);
        let min_se_nw = cmp::min(se, nw);
        let min_ne_sw = cmp::min(ne, sw);
        n -= min_n_s;
        s -= min_n_s;
        se -= min_se_nw;
        nw -= min_se_nw;
        ne -= min_ne_sw;
        sw -= min_ne_sw;

        let min_n_s = cmp::min(n, s);
        n -= min_n_s;
        s -= min_n_s;
    } 

    res.insert(N, n);
    res.insert(NE, ne);
    res.insert(SE, se);
    res.insert(S, s);
    res.insert(SW, sw);
    res.insert(NW, nw);
    res
}

fn split(dirs: &[Direction]) -> HashMap<Direction, i32> {
    let mut res = HashMap::new();
    res.insert(N, 0);
    res.insert(NE, 0);
    res.insert(SE, 0);
    res.insert(S, 0);
    res.insert(SW, 0);
    res.insert(NW, 0);
    for dir in dirs {
        let count = res.entry(dir.clone()).or_insert(0);
        *count += 1;
    }
    res
}

fn load(input: &str) -> Vec<Direction> {
    input.split(",")
         .map(|it| match it {
             "n" => N,
             "ne" => NE,
             "se" => SE,
             "s" => S,
             "sw" => SW,
             "nw" => NW,
             x => panic!("unknown direction: {}", x),
         })
         .collect()
}

fn day11_2() {
    let file_name = "input/day11.txt";
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut input = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut input).expect(&err_read);

    let mut max_d = 0;
    let dirs = load(&input);
    for i in 1..dirs.len() {
        //println!("{:?}", &dirs[0..i]);
        let split = split(&dirs[0..i]);
        let res = simplify(&split);
        let d: i32 = res.values().sum();
        if d > max_d {
            println!("{:?} -> {}", res, d);
            max_d = d;
        }
    }
}

