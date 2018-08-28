use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use day8::Operation::{INC,DEC};
use day8::Test::{LT,LTEQ,EQ,NEQ,GTEQ,GT};

#[derive(Debug)]
enum Operation {
    INC,
    DEC,
}

#[derive(Debug)]
enum Test {
    LT,
    LTEQ,
    EQ,
    NEQ,
    GTEQ,
    GT,
}

#[derive(Debug)]
struct Instruction {

    reg: String,
    op: Operation,
    val: i32,

    reg_cmp: String,
    op_cmp: Test,
    val_cmp: i32,
}

struct Day8 {
    instructions: Vec<Instruction>,
    regs: HashMap<String,i32>,
}

pub fn day8() {
    let mut day8 = Day8::new();
    day8.run();
}

impl Day8 {

    pub fn new() -> Day8 {
        Day8 {
            instructions: Vec::new(),
            regs: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        self.load("input/day8.txt");
        for instruction in &self.instructions {
            let reg_cmp = &instruction.reg_cmp;
            let val_cmp = instruction.val_cmp;
            let apply = match instruction.op_cmp {
                LT => self.regs[reg_cmp] < val_cmp,
                LTEQ => self.regs[reg_cmp] <= val_cmp,
                EQ => self.regs[reg_cmp] == val_cmp,
                NEQ => self.regs[reg_cmp] != val_cmp,
                GTEQ => self.regs[reg_cmp] >= val_cmp,
                GT => self.regs[reg_cmp] > val_cmp,
            };
            if !apply {
                continue;
            }
            let reg = instruction.reg.clone();
            let val = instruction.val;
            match instruction.op {
                INC => self.regs.entry(reg).and_modify(|v| *v += val),
                DEC => self.regs.entry(reg).and_modify(|v| *v -= val),
            };
        }
        let max = self.regs.values().max().unwrap();
        println!("max = {}", max);
    }

    fn load(&mut self, file: &str) {
        let err_msg = format!("Failed to open {}", &file);
        let file = File::open(file).expect(&err_msg);
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let items: Vec<&str> = line.split(" ").collect();
            assert_eq!(items.len(), 7);
            //println!("{:?}", items);
            let reg = items[0].to_string();
            let op = match items[1] {
                "inc" => INC,
                "dec" => DEC,
                _ => panic!("unknown operation"),
            };
            let val = items[2].to_string().parse::<i32>().unwrap();
            let reg_cmp = items[4].to_string();
            let op_cmp = match items[5] {
                "<" => LT,
                "<=" => LTEQ,
                "==" => EQ,
                "!=" => NEQ,
                ">=" => GTEQ,
                ">" => GT,
                _ => panic!("unknown comparison"),
            };
            let val_cmp = items[6].to_string().parse::<i32>().unwrap();
            self.instructions.push(Instruction {
                reg,
                op,
                val,
                reg_cmp,
                op_cmp,
                val_cmp,
            });
            println!("{:?}", self.instructions.iter().last().unwrap());
        }
        for instruction in &self.instructions {
            self.regs.entry(instruction.reg.clone()).or_insert(0);
            self.regs.entry(instruction.reg_cmp.clone()).or_insert(0);
        }
    }
}
