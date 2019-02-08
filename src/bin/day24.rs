use std::fmt;

const INPUT: &'static str = include_str!("../../input/Day24.txt");
const INPUT_TEST: &'static str = include_str!("../../input/Day24-test.txt");

#[derive(Clone, Copy)]
struct Component {
    i: usize,
    p0: i32,
    p1: i32
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.p0, self.p1)?;
        Ok(())
    }
}

impl Component {
    fn from_text(i:usize, text: &str) -> Component {
        let mut ports = text.split("/").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        ports.sort();
        Component {
            i,
            p0: ports[0],
            p1: ports[1],
        }
    }
}

fn from_text(text: &str) -> Vec<Component> {
    let mut components = Vec::new();
    let mut i = 0;
    for line in text.lines() {
        let component = Component::from_text(i, line);
        components.push(component);
        i += 1;
    }
    components
}

fn combine_rec(mut v: Vec<Component>, port: i32, mut bag: Vec<Component>, res: &mut Vec<Component>, max_strength: &mut i32) {
        let matching_components = bag.iter().filter(|c| c.p0 == port || c.p1 == port).cloned().collect::<Vec<Component>>();
        if port == 3 {
            //println!(">>>>>>>>>>>>>>>>>>>>> {:?}", matching_components);        
        }
        for c in matching_components.clone() {
            let next_port = if c.p0 == port { c.p1 } else { c.p0 };
            v.push(c);
            bag.retain(|x| x.i != c.i);
            let strength = v.iter().fold(0, |sum, c| sum + c.p0 + c.p1);
            if strength > *max_strength {
                println!("{} -> {:?}", strength, v);
                //println!("{:?}", matching_components);
                *max_strength = strength;
                *res = v.clone();
            }
            combine_rec(v.clone(), next_port, bag.clone(), res, max_strength);
            bag.push(v.pop().unwrap());
        }

}

fn compute_strongest(components: &Vec<Component>) -> (i32, Vec<Component>) {
    let mut res = vec![];
    let mut max_strength = 0;
    let start_components = components.iter().filter(|c| c.p0 == 0).cloned().collect::<Vec<Component>>();
    for start_comp in start_components {
        let mut bag = components.iter().filter(|x| x.i != start_comp.i).cloned().collect();
        let mut v = vec![start_comp];
        let mut port = start_comp.p1;
        combine_rec(v, port, bag, &mut res, &mut max_strength);
    }
    (max_strength, res)
}

fn main() {
    day24();
}

fn day24() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    let components = from_text(INPUT_TEST);
    println!("{:?}", components);
    let strongest = compute_strongest(&components);
    println!("***** {} -> {:?}", strongest.0, strongest.1);

    let components = from_text(INPUT);
    println!("{:?}", components);
    let strongest = compute_strongest(&components);
    println!("***** {} -> {:?}", strongest.0, strongest.1);
}

fn part2() {
    println!("Part 2");
}

