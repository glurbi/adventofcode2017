use std::fs::File;
use std::io::prelude::*;
use pest::Parser;
use pest::iterators::Pairs;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("day9.pest");

#[derive(Parser)]
#[grammar = "day9.pest"]
pub struct StreamParser;

pub fn day9() {

    let examples = vec![
        "{}",
        "{{{}}}",
        "{{},{}}",
        "{{{},{},{{}}}}",
        "{<{},{},{{}}>}",
        "{<a>,<a>,<a>,<a>}",
        "{{<a>},{<a>},{<a>},{<a>}}",
        "{{<!>},{<!>},{<!>},{<a>}}",
        "{{<ab>},{<ab>},{<ab>},{<ab>}}",
        "{{<!!>},{<!!>},{<!!>},{<!!>}}",
        "{{<a!>},{<a!>},{<a!>},{<ab>}}"
    ];

    for ex in examples.iter() {
        println!("{:?}", StreamParser::parse(Rule::stream, ex));
    }

    for ex in examples {
        println!("----------------");
        let mut score = 0;
        let mut garbage = 0;
        let pairs = StreamParser::parse(Rule::stream, ex).unwrap_or_else(|e| panic!("{}", e));
        visit(pairs, 1, &mut score, &mut garbage);
        println!("{} -> {}, {}", ex, score, garbage);
    }

    let file_name = "input/day9.txt";
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut stream = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut stream).expect(&err_read);

    println!("----------------");
    let mut score = 0;
    let mut garbage = 0;
    let pairs = StreamParser::parse(Rule::stream, &stream).unwrap_or_else(|e| panic!("{}", e));
    visit(pairs, 1, &mut score, &mut garbage);
    println!("{} -> {}, {}", "...", score, garbage);
}

fn visit(pairs: Pairs<Rule>, depth: i32, score: &mut i32, garbage: &mut i32) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::group => {
                *score += depth;
                visit(pair.into_inner(), depth + 1, score, garbage);
            },
            Rule::not_escape => {
                *garbage += 1;
                continue;
            },
            _ => unreachable!()
        };
    }
}
