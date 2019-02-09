use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct State {
    w0: i8,
    m0: i8,
    s0: Option<Rc<RefCell<State>>>,
    w1: i8,
    m1: i8,
    s1: Option<Rc<RefCell<State>>>,
}

#[derive(Debug)]
struct Machine {
    p: i64,
    v: Vec<i8>,
    s: Rc<RefCell<State>>,
}

impl Machine {

    fn new(s: Rc<RefCell<State>>) -> Machine {
        let v = vec![0; 100000];
        let p = v.len() as i64 / 2;
        Machine { p, v, s }
    }

    fn checksum(&self) -> i64 {
        self.v.iter().fold(0, |acc, x| acc + *x as i64)
    }

    fn step(&mut self) {
        if self.v[self.p as usize] == 0 {
            self.v[self.p as usize] = self.s.borrow().w0;
            self.p += self.s.borrow().m0 as i64;
            let r = self.s.borrow().s0.as_ref().unwrap().clone();
            self.s = r;
        } else {
            self.v[self.p as usize] = self.s.borrow().w1;
            self.p += self.s.borrow().m1 as i64;
            let r = self.s.borrow().s1.as_ref().unwrap().clone();
            self.s = r;
        }
    }

}

fn main() {
    day25();
}

fn day25() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");
    let a = Rc::new(RefCell::new(State { w0: 1, m0: 1, s0: None, w1: 0, m1: -1, s1: None }));
    let b = Rc::new(RefCell::new(State { w0: 1, m0: -1, s0: None, w1: 1, m1: 1, s1: None }));
    a.borrow_mut().s0 = Some(b.clone());
    a.borrow_mut().s1 = Some(b.clone());
    b.borrow_mut().s0 = Some(a.clone());
    b.borrow_mut().s1 = Some(a.clone());
    let mut m = Machine::new(a.clone());
    for _ in 0..6 {
        m.step();
    }
    println!("{}", m.checksum());
    //println!("{:?}", m.v);

    let a = Rc::new(RefCell::new(State { w0: 1, m0: 1, s0: None, w1: 0, m1: 1, s1: None }));
    let b = Rc::new(RefCell::new(State { w0: 0, m0: -1, s0: None, w1: 1, m1: -1, s1: None }));
    let c = Rc::new(RefCell::new(State { w0: 1, m0: -1, s0: None, w1: 0, m1: 1, s1: None }));
    let d = Rc::new(RefCell::new(State { w0: 1, m0: -1, s0: None, w1: 1, m1: 1, s1: None }));
    let e = Rc::new(RefCell::new(State { w0: 1, m0: -1, s0: None, w1: 0, m1: -1, s1: None }));
    let f = Rc::new(RefCell::new(State { w0: 1, m0: 1, s0: None, w1: 0, m1: -1, s1: None }));
    a.borrow_mut().s0 = Some(b.clone());
    a.borrow_mut().s1 = Some(f.clone());
    b.borrow_mut().s0 = Some(b.clone());
    b.borrow_mut().s1 = Some(c.clone());
    c.borrow_mut().s0 = Some(d.clone());
    c.borrow_mut().s1 = Some(c.clone());
    d.borrow_mut().s0 = Some(e.clone());
    d.borrow_mut().s1 = Some(a.clone());
    e.borrow_mut().s0 = Some(f.clone());
    e.borrow_mut().s1 = Some(d.clone());
    f.borrow_mut().s0 = Some(a.clone());
    f.borrow_mut().s1 = Some(e.clone());
    let mut m = Machine::new(a.clone());
    for _ in 0..12964419 {
        m.step();
    }
    println!("{}", m.checksum());
    //println!("{:?}", m.v);
}

fn part2() {
    println!("Part 2");
}

