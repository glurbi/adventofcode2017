const INPUT: &'static str = include_str!("../../input/Day5.txt");

fn main() {
    day1();
}

fn day1() {
    run1();
    run2();
}

fn run1() {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let mut v: Vec<i32> = Vec::new();
            
    for s in lines {
        v.push(s.parse::<i32>().unwrap());
    }

    let mut pos = 0 as i32;
    let mut count = 0 as i32;

    while pos >= 0 && pos < v.len() as i32
    {
        v[pos as usize] += 1;
        pos += v[pos as usize] - 1;
        count += 1;
    }

    println!("{}", count);
}

fn run2() {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let mut v: Vec<i32> = Vec::new();
            
    for s in lines {
        v.push(s.parse::<i32>().unwrap());
    }

    let mut pos = 0 as i32;
    let mut count = 0 as i32;

    while pos >= 0 && pos < v.len() as i32 {
        let inc = if v[pos as usize] < 3 { 1 } else { -1 };
        v[pos as usize] += inc;
        pos += v[pos as usize] - inc;
        count += 1;
    }

    println!("{}", count);
}
