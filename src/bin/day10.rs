fn main() {
    day10();
}

fn day10() {
    day10_1();
    day10_2();
}

fn day10_1() {

    let mut pos = 0;
    let mut skip = 0;
    let mut ex: Vec<u8> = vec![0,1,2,3,4];
    process(&mut ex, &vec![3,4,1,5], &mut pos, &mut skip);
    //println!("{}", ex[0] * ex[1]);

    let mut pos = 0;
    let mut skip = 0;
    let mut list: Vec<u8> = zero_to_255();
    let lengths = vec![212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164];
    process(&mut list, &lengths, &mut pos, &mut skip);
    //println!("{}", list[0] * list[1]);
}

fn day10_2() {

    let mut list: Vec<u8> = zero_to_255();
    let ex: Vec<u8> = str2lengths(&"");
    let hash = compute_hash(&mut list, &ex);
    println!("{}", hash);
    //println!("{:?}", ex);

    let mut list: Vec<u8> = zero_to_255();
    let lengths: Vec<u8> = str2lengths(&"212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164");
    //println!("{:?}", lengths);
    let hash = compute_hash(&mut list, &lengths);
    println!("{}", hash);
}

fn zero_to_255() -> Vec<u8> {
    (0..256).map(|i| i as u8)
            .collect()
}

fn compute_hash(list: &mut Vec<u8>, lengths: &Vec<u8>) -> String {
    let mut pos = 0;
    let mut skip = 0;
    for _ in 1..65 {
        process(list, lengths, &mut pos, &mut skip);
    }
    let mut hash: Vec<u8> = Vec::new();
    for i in 0..16 {
        let sub = &list[i*16..(i+1)*16];
        let xor = sub.iter().map(|x| *x as u8).fold(0, |acc, x| acc ^ x);
        hash.push(xor);
    }
    let hash: Vec<String> = hash.iter().map(|b| format!("{:02x}", b)).collect();
    hash.join("")
}

fn str2lengths(s: &str) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend(s.as_bytes());
    res.extend(&[17, 31, 73, 47, 23]);
    res
}

fn process(list: &mut Vec<u8>, lengths: &Vec<u8>, pos: &mut u8, skip: &mut u8) {
    for length in lengths {
        let p: usize = *pos as usize;
        let indices: Vec<u8> = (0..*length).map(|i| i as usize)
                                           .map(|i| (p + i) % list.len())
                                           .map(|i| i as u8)
                                           .collect();
        let mut values: Vec<u8> = indices.iter()
                                         .map(|i| list[*i as usize])
                                         .collect();
        values.reverse();
        for i in 0..values.len() {
            list[indices[i] as usize] = values[i as usize];
        }
        *pos = ((p + *length as usize + *skip as usize) as usize % list.len()) as u8;
        *skip = ((*skip as usize) + 1) as u8;
    }
}