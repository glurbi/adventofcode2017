#[derive(Debug)]
struct Generator {
    n: u64,
    f: u64,
} 

impl Generator {

    fn new(init: u64, fact: u64) -> Generator {
        Generator {
            n: init,
            f: fact,
        }
    }

    fn next(&mut self) -> u64 {
        let d = 2147483647;
        self.n = (self.f * self.n) % d;
        self.n
    } 

    fn judge(gen_a: &mut Generator, gen_b: &mut Generator, iterations: u64) -> u64 {
        let mut count = 0;
        for _ in 0..iterations {
            let next_a = gen_a.next();        
            let next_b = gen_b.next();
            //println!("{} {}", next_a, next_b);
            if next_a & 0xffff == next_b & 0xffff {
                count += 1;
            }
        }
        count
    }
}

pub fn day15() {
    day15_1();
    day15_2();
}

fn day15_1() {
    println!("Part 1");

    let mut gen_a = Generator::new(65, 16807);
    let mut gen_b = Generator::new(8921, 48271);
    println!("{}", Generator::judge(&mut gen_a, &mut gen_b, 40000000));

    let mut gen_a = Generator::new(679, 16807);
    let mut gen_b = Generator::new(771, 48271);
    println!("{}", Generator::judge(&mut gen_a, &mut gen_b, 40000000));
}

#[derive(Debug)]
struct Generator2 {
    n: u64,
    f: u64,
    m: u64,
} 

impl Generator2 {

    fn new(init: u64, fact: u64, mult: u64) -> Generator2 {
        Generator2 {
            n: init,
            f: fact,
            m: mult,
        }
    }

    fn next(&mut self) -> u64 {
        while {
            let d = 2147483647;
            self.n = (self.f * self.n) % d;
            self.n % self.m != 0
        } {}
        self.n
    } 

    fn judge(gen_a: &mut Generator2, gen_b: &mut Generator2, iterations: u64) -> u64 {
        let mut count = 0;
        for _ in 0..iterations {
            let next_a = gen_a.next();        
            let next_b = gen_b.next();
            //println!("{} {}", next_a, next_b);
            if next_a & 0xffff == next_b & 0xffff {
                count += 1;
            }
        }
        count
    }

}

fn day15_2() {
    println!("Part 2");

    let mut gen_a = Generator2::new(65, 16807, 4);
    let mut gen_b = Generator2::new(8921, 48271, 8);
    //println!("{}", Generator2::judge(&mut gen_a, &mut gen_b, 1056));
    println!("{}", Generator2::judge(&mut gen_a, &mut gen_b, 5000000));

    let mut gen_a = Generator2::new(679, 16807, 4);
    let mut gen_b = Generator2::new(771, 48271, 8);
    println!("{}", Generator2::judge(&mut gen_a, &mut gen_b, 5000000));
}
