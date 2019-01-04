fn main() {
    day6();
}

fn day6() {
    let mut mem = vec![ 11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11 ];
    //let mut mem = vec![ 0, 2, 7, 0 ];
    let mut cycles = 0;
    let mut mems: Vec<Vec<i32>> = Vec::new();
    while !mems.contains(&mem) {
        mems.push(mem.to_vec());
        mem = redistribute(&mem);
        cycles += 1;
    }
    println!("{:?}", mem);
    let pos = mems.iter().position(|v| *v == mem);
    match pos {
        Some(v) => println!("infinite cycle = {}", cycles - v),
        None => ()
    } 
    println!("{} cycles", cycles);

}

fn redistribute(mem: &Vec<i32>) -> Vec<i32> {
    let mut out = mem.to_vec();
    let (mut index, mut value) = find_bank(&mem);
    out[index] = 0;
    while value > 0 {
        index = (index + 1) % out.len();
        out[index] += 1;
        value -= 1;
    }
    out
}

fn find_bank(mem: &Vec<i32>) -> (usize, i32) {
    let mut index = 0;
    let mut value = mem[0];
    for (i, &v) in mem.iter().enumerate() {
        if v > value {
            index = i;
            value = v;
        }
    }
    (index, value)
}