pub fn day17() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");
    let mut buf: Vec<usize> = vec![0];
    let mut pos: usize = 0;
    println!("{:?}, {}", buf, pos);
    let steps: usize = 382;
    for val in 1..2018 {
        pos = (pos + steps) % buf.len() + 1;
        buf.insert(pos, val);
        //println!("{:?}, {}", buf, pos);
    }
    println!("{:?}, {}", &buf[pos-3..pos+4], pos);
    println!("=> {}", buf[pos+1]);
}

fn part2() {

    println!("Part 2");

}


