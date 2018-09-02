pub fn day10() {

    let mut ex: Vec<usize> = vec![0,1,2,3,4];
    process(&mut ex, &vec![3,4,1,5]);
    println!("{}", ex[0] * ex[1]);

    let mut list: Vec<usize> = (0..256).collect();
    let lengths = vec![212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164];
    process(&mut list, &lengths);
    println!("{}", list[0] * list[1]);
}

fn process(list: &mut Vec<usize>, lengths: &Vec<usize>) {
    let mut pos = 0;
    let mut skip = 0;
    for length in lengths {
        let cur_pos = pos;
        let indices: Vec<usize> = (0..*length).map(|i| (cur_pos + i) % (list.len())).collect();
        let mut values: Vec<usize> = indices.iter().map(|i| list[*i]).collect();
        values.reverse();
        for i in 0..values.len() {
            list[indices[i]] = values[i];
        }
        pos = (pos + length + skip) % (list.len());
        skip += 1;
    }
}