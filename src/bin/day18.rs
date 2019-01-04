use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

use Instruction::{SND,SET,ADD,MUL,MOD,RCV,JGZ};

fn main() {
    day18();
}

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
    id: i32,
    regs: HashMap<char, i64>,
    ip: i64,
    freq: i64,
    send_count: i64,
    input_q: VecDeque<i64>,
}

impl VirtualMachine {
    fn new(id: i32) -> VirtualMachine {
        VirtualMachine {
            id,
            regs: HashMap::new(),
            ip: 0,
            freq: 0,
            send_count: 0,
            input_q: VecDeque::new(),
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

impl Prog {

    fn step2(&self, vm: &mut VirtualMachine, other_vm: &mut VirtualMachine) -> Result<(),()> {

        if self.is_valid_ip(&vm) {
            return Err(());
        }

        let instr = &self.instructions[vm.ip as usize];
        //println!("{:?}", &instr);

        match instr {
            SND(x) => {
                let val_x = vm.regs[&x];
                other_vm.input_q.push_back(val_x);
                vm.send_count += 1;
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
                if vm.input_q.len() == 0 {
                    return Err(());
                }
                *vm.regs.entry(*x).or_insert(0) = vm.input_q.pop_front().unwrap();
                vm.ip += 1;
            },
            JGZ(x, y) => {
                let val_x = match x.to_string().parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => vm.regs[x],
                };
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
        //println!("{:?}", &vm);

        Ok(())
    }

    fn run2(&self, vm0: &mut VirtualMachine, vm1: &mut VirtualMachine) -> Result<(),()> {
        loop {
            loop {
                let res0 = self.step2(vm0, vm1);
                let res1 = self.step2(vm1, vm0);

                if res0.is_err() && res1.is_err() {
                    println!("-----------");
                    println!("{:?}", &vm0);
                    println!("{:?}", &vm1);
                    return Err(());
                }
            }
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
    let mut vm = VirtualMachine::new(0);
    prog.run(&mut vm).expect("Failed to run part 1 test");

    let input = read_to_string("input/Day18.txt");
    let prog = Prog::from_text(&input);
    let mut vm = VirtualMachine::new(0);
    prog.run(&mut vm).expect("Failed to run part 1");
}

fn part2() {
    println!("Part 2");
    let input = read_to_string("input/Day18.txt");
    let prog = Prog::from_text(&input);
    let mut vm0 = VirtualMachine::new(0);
    let mut vm1 = VirtualMachine::new(1);
    *vm1.regs.entry('p').or_insert(0) = 1;
    prog.run2(&mut vm0, &mut vm1).expect("Failed to run part 2");
}

fn read_to_string(file_name: &str) -> String {
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut content = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut content).expect(&err_read);
    content
}
