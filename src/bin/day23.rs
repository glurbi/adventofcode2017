use InstructionType::{SET,SUB,MUL,JNZ};

const INPUT: &'static str = include_str!("../../input/Day23.txt");
const INPUT_OPT: &'static str = include_str!("../../input/Day23-opt.txt");

fn main() {
    day23();
}

#[derive(Debug)]
enum InstructionType { SET, SUB, MUL, JNZ }

#[derive(Debug)]
struct Instruction {
    instr_type: InstructionType,
    x_val: Option<i64>,
    x_reg: Option<usize>,
    y_val: Option<i64>,
    y_reg: Option<usize>,
}

fn to_instruction(line: &str) -> Instruction {
    let split: Vec<&str> = line.split(" ").collect();
    let x = split[1].chars().next().unwrap();
    let y = split[2].to_string();
    let x_val = x.to_string().parse::<i64>().ok();
    let x_reg = match x_val {
        Some(_) => None,
        None => Some(x as usize - 'a' as usize),
    };
    let y_val = y.parse::<i64>().ok();
    let y_reg = match y_val {
        Some(_) => None,
        None => Some(y.chars().next().unwrap() as usize - 'a' as usize),
    };
    let instr_type = match split[0] {
        "set" => SET,
        "sub" => SUB,
        "mul" => MUL,
        "jnz" => JNZ,
        _ => panic!("Panic!"),
    };
    Instruction {
        instr_type,
        x_val,
        x_reg,
        y_val,
        y_reg,
    }
}

#[derive(Debug)]
struct Prog {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct VirtualMachine {
    regs: [i64; 8],
    ip: i64,
    mul_count: usize,
    step_count: usize,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            regs: [0; 8],
            ip: 0,
            mul_count: 0,
            step_count: 0,
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
        //println!("{:?}", vm);
        vm.ip < 0 || (vm.ip as usize) < self.instructions.len()
    }

    fn step(&self, vm: &mut VirtualMachine) -> bool {

        if !self.is_valid_ip(vm) {
            return false
        }

        vm.step_count += 1;

        let i = &self.instructions[vm.ip as usize];
        match i.instr_type {
            SET => {
                vm.regs[i.x_reg.unwrap()] = match i.y_reg {
                    Some(r) => vm.regs[r],
                    None => i.y_val.unwrap(),
                };
                vm.ip += 1;
            },
            SUB => {
                vm.regs[i.x_reg.unwrap()] -= match i.y_reg {
                    Some(r) => vm.regs[r],
                    None => i.y_val.unwrap(),
                };
                vm.ip += 1;
            },
            MUL => {
                vm.regs[i.x_reg.unwrap()] *= match i.y_reg {
                    Some(r) => vm.regs[r],
                    None => i.y_val.unwrap(),
                };
                vm.mul_count += 1;
                vm.ip += 1;
            },
            JNZ => {
                let val_x = match i.x_reg {
                    Some(r) => vm.regs[r],
                    None => i.x_val.unwrap(),
                };
                let jmp = match i.y_reg {
                    Some(r) => vm.regs[r],
                    None => i.y_val.unwrap(),
                };
                if val_x != 0 {
                    vm.ip += jmp;
                } else {
                    vm.ip += 1;
                }
            },
        }
        //println!("{:?} ---- {:?}", &i, &vm);

        true
    }

    fn run(&self, vm: &mut VirtualMachine) {
        while self.step(vm) {
            //if vm.step_count % 1000000 == 0 {
                println!("{:?}", &vm);
            //}
        }
        println!("{:?}", &vm);
    }
}

fn day23() {
    //part1();
    part2();
}

fn part1() {
    println!("Part 1");
    let prog = Prog::from_text(&INPUT);
    println!("{:?}", prog);
    let mut vm = VirtualMachine::new();
    prog.run(&mut vm);
    println!("step count={}", vm.step_count);
    println!("mul count={}", vm.mul_count);
}

fn part2() {
    println!("Part 2");
    let prog = Prog::from_text(&INPUT);
    //let prog = Prog::from_text(&INPUT_OPT);
    println!("{:?}", prog);
    let mut vm = VirtualMachine::new();
    vm.regs[0] = 1;
    prog.run(&mut vm);
    println!("step count={}", vm.step_count);
    println!("mul count={}", vm.mul_count);
    println!("reg h={}", vm.regs[7]);
}

