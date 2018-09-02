pub fn day10() {

    let mut ex = vec![0,1,2,3,4];
    process(&mut ex, &vec![3,4,1,5]);
    println!("{}", ex[0] * ex[1]);

    let mut list: Vec<i32> = (0..256).collect();
    let lengths = vec![212,254,178,237,2,0,1,54,167,92,117,125,255,61,159,164];
    process(&mut list, &lengths);
    println!("{}", list[0] * list[1]);
}

fn process(list: &mut Vec<i32>, lengths: &Vec<i32>) {
    let mut pos = 0;
    let mut skip = 0; 
    for length in lengths {
        //println!("{}", pos);
        let cur_pos = pos;
        let indices: Vec<i32> = (0..*length).collect();
        let indices: Vec<i32> = indices.iter().map(|i| (cur_pos + i) % (list.len() as i32)).collect();
        let values: Vec<i32> = indices.iter().map(|i| list[*i as usize]).collect();
        let mut values = values.clone();
        values.reverse();
        for i in 0..values.len() {
            list[indices[i] as usize] = values[i];
        }
        //println!("{:?}", list);
        pos = (pos + length + skip) % (list.len() as i32);
        skip += 1;
    }
}