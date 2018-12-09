use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use day18::Instruction::{SND,SET,ADD,MUL,MOD,RCV,JGZ};

#[derive(Debug)]
enum Instruction {
    SND(char),
    SET(char, String),
    ADD(char, String),
    MUL(char, String),
    MOD(char, String),
    RCV(char),
    JGZ(char, String),
}

fn to_instruction(line: &str) -> Instruction {
    let split: Vec<&str> = line.split(" ").collect();
    let x = split[1].chars().next().unwrap();
    let y = match split.len() {
        3 => split[2].to_string(),
        _ => "".to_string(),
    };
    match split[0] {
        "snd" => SND(x),
        "set" => SET(x, y),
        "add" => ADD(x, y),
        "mul" => MUL(x, y),
        "mod" => MOD(x, y),
        "rcv" => RCV(x),
        "jgz" => JGZ(x, y),
        _ => panic!("Panic!"),
    }
}

#[derive(Debug)]
struct Prog {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct VirtualMachine {
    regs: HashMap<char, i64>,
    ip: i64,
    freq: i64,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            regs: HashMap::new(),
            ip: 0,
            freq: 0,
        }
    }
}

impl Prog {
    fn from_text(text: &str) -> Prog {
        let mut instructions = Vec::new();
        for line in text.lines() {
            let instr = to_instruction(line);
            //println!("*{:?}", &instr);
            instructions.push(instr);
        }
        Prog {
            instructions,
        }
    }

    fn is_valid_ip(&self, vm: &VirtualMachine) -> bool {
        vm.ip < 0 || vm.ip > self.instructions.len() as i64
    }

    fn step(&self, vm: &mut VirtualMachine) -> Result<(),()> {

        if self.is_valid_ip(&vm) {
            return Err(());
        }

        let instr = &self.instructions[vm.ip as usize];
        match instr {
            SND(x) => {
                let val_x = vm.regs[&x];
                vm.freq = val_x;
                vm.ip += 1;
            },
            SET(x, y) => {
                let val_y = match y.parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => vm.regs[&y.chars().next().unwrap()],
                };
                *vm.regs.entry(*x).or_insert(0) = val_y;
                vm.ip += 1;
            },
            ADD(x, y) => {
                let val_y = match y.parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => vm.regs[&y.chars().next().unwrap()],
                };
                *vm.regs.entry(*x).or_insert(0) += val_y;
                vm.ip += 1;
            },
            MUL(x, y) => {
                let val_y = match y.parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => vm.regs[&y.chars().next().unwrap()],
                };
                *vm.regs.entry(*x).or_insert(0) *= val_y;
                vm.ip += 1;
            },
            MOD(x, y) => {
                let val_x = vm.regs[&x];
                let val_y = match y.parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => vm.regs[&y.chars().next().unwrap()],
                };
                *vm.regs.entry(*x).or_insert(0) = val_x % val_y;
                vm.ip += 1;
            },
            RCV(x) => {
                let val_x = vm.regs[&x];
                vm.ip += 1;
                if val_x > 0 {
                    println!("RECOVER -> {}", vm.freq);
                    return Err(());
                }
            },
            JGZ(x, y) => {
                let val_x = vm.regs[&x];
                let val_y = match y.parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => vm.regs[&y.chars().next().unwrap()],
                };
                if val_x > 0 {
                    vm.ip += val_y;
                } else {
                    vm.ip += 1;
                }
            },
        }
        println!("{:?} ---- {:?}", &instr, &vm);

        Ok(())
    }

    fn run(&self, vm: &mut VirtualMachine) -> Result<(),()> {
        loop {
            self.step(vm)?;
        } 
    }
}

pub fn day18() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    let input = read_to_string("input/Day18-test.txt");
    let prog = Prog::from_text(&input);
    //println!("{:?}", prog);
    let mut vm = VirtualMachine::new();
    prog.run(&mut vm);

    let input = read_to_string("input/Day18.txt");
    let prog = Prog::from_text(&input);
    let mut vm = VirtualMachine::new();
    prog.run(&mut vm);
}

fn part2() {
    println!("Part 2");
}

fn read_to_string(file_name: &str) -> String {
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut content = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut content).expect(&err_read);
    content
}
