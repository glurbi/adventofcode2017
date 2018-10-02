pub fn day14() {
    //day14_1();
    day14_2();
}

fn count_used_squares(key: &str) -> u32 {
    (0..128).map(|i| format!("{}-{}", key, i.to_string()))
            .map(|k| knot_hash(&k))
            .collect::<Vec<String>>()
            .join("")
            .chars()
            .map(|c| u32::from_str_radix(&c.to_string(), 16).unwrap())
            .map(|i| i.count_ones())
            .sum()
}

const S: i32 = 128;

fn offset(x:i32, y:i32) -> usize {
    let x = if x < 0 { 0 } else { x };
    let x = if x >= S { S } else { x };
    let y = if y < 0 { 0 } else { y };
    let y = if y >= S { S } else { y };
    (y * S + x) as usize
}

#[derive(Debug)]
struct Disk {
    key: String,
    bits: Vec<u8>,
}

impl Disk {
    fn from_key(key: &str) -> Disk {
        Disk {
            key: key.to_string(),
            bits: (0..128).map(|i| format!("{}-{}", key, i.to_string()))
                          .map(|k| knot_hash(&k))
                          .collect::<String>()
                          .chars()
                          .map(|c| u32::from_str_radix(&c.to_string(), 16).unwrap())
                          .map(|i| format!("{:04b}", i))
                          .collect::<String>()
                          .chars()
                          .map(|c| u8::from_str_radix(&c.to_string(), 2).unwrap())
                          .collect::<Vec<u8>>()
        }
    }

    fn count_groups(&self) -> u32 {
        let mut visited = Vec::new();
        let mut count = 0;
        for y in 0..S {
            for x in 0..S {
                if visited[offset(x,y)] {
                    continue;
                }
                visited[offset(x,y)] = true;
                if self.bits[offset(x,y)] == 0 {
                    continue;
                }
                count += 1;
                self.visit_group(x, y, &mut visited);
            }
        }
        count
    }

    fn visit_group(&self, x:i32, y:i32, visited: &mut Vec<bool>) {
        if visited[offset(x,y)] {
            return;
        }
        visited[offset(x,y)] = true;
        if self.bits[offset(x,y)] == 0 {
            return;
        }
        self.visit_group(x+1, y, visited);
        self.visit_group(x-1, y, visited);
        self.visit_group(x, y+1, visited);
        self.visit_group(x, y-1, visited);
    }
}

fn day14_1() {
    let key = "flqrgnkx";
    let ones = count_used_squares(key);
    println!("{} has {} used squares", key, ones);

    let key = "hfdlxzhv";
    let ones = count_used_squares(key);
    println!("{} has {} used squares", key, ones);
}

fn day14_2() {
    let key = "flqrgnkx";
    let disk = Disk::from_key(key);
    //println!("{:?}", disk);
    let count = disk.count_groups();
    println!("{}", count);
}

fn knot_hash(key: &str) -> String {

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

    fn zero_to_255() -> Vec<u8> {
        (0..256).map(|i| i as u8)
                .collect()
    }

    let mut list: Vec<u8> = zero_to_255();
    let ex: Vec<u8> = str2lengths(key);
    compute_hash(&mut list, &ex)
}
