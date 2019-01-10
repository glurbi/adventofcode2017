#[macro_use]
extern crate nom;

use std::collections::HashMap;
use nom::{digit};

const INPUT: &'static str = include_str!("../../input/Day20.txt");

fn main() {
    day1();
}

fn day1() {
    run1();
    run2();
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

impl Particle {

    fn step(&mut self) {
        self.v.x += self.a.x;
        self.v.y += self.a.y;
        self.v.z += self.a.z;
        self.p.x += self.v.x;
        self.p.y += self.v.y;
        self.p.z += self.v.z;
    }

    fn distance(&self) -> usize {
        (self.p.x.abs() + self.p.y.abs() + self.p.z.abs()) as usize
    }

}

named!(
    integer<&str, i64>, flat_map!(
    recognize!(
        tuple!(
            opt!(char!('-')),
            digit
    )),
    parse_to!(i64)
));

named!(
    vec3<&str, Vec3>,
    do_parse!(
        x: integer >>
        tag!(",") >>
        y: integer >>
        tag!(",") >>
        z: integer >>
        (Vec3 { x, y, z }))
);

named!(
    position<&str, Vec3>,
    do_parse!(
        tag!("p=<") >>
        v: vec3 >>
        tag!(">") >>
        (v)
    )
);

named!(
    velocity<&str, Vec3>,
    do_parse!(
        tag!("v=<") >>
        v: vec3 >>
        tag!(">") >>
        (v)
    )
);

named!(
    acceleration<&str, Vec3>,
    do_parse!(
        tag!("a=<") >>
        v: vec3 >>
        tag!(">") >>
        (v)
    )
);

named!(
    particle<&str, Particle>,
    do_parse!(
        p: position >>
        tag!(", ") >>
        v: velocity >>
        tag!(", ") >>
        a: acceleration >>
        (Particle { p, v, a }))
);

fn read_particles() -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    for line in INPUT.lines() {
        let particle = particle(line.trim());
        particles.push(particle.unwrap().1);
    }
    particles
}

fn run1() {

    let mut particles = read_particles();

    let mut closest_i = std::usize::MAX;
    let mut closest_d = std::usize::MAX;
    for _ in 0..1000 {

        for i in 0..particles.len() {
            let particle = &mut particles[i];
            particle.step();
        }

        let mut dist = std::usize::MAX;
        let mut index = std::usize::MAX;
        for i in 0..particles.len() {
            let particle = &particles[i];
            if particle.distance() < dist {
                index = i;
                dist = particle.distance();
            }
        }

        if closest_i != index {
            println!("i={}, dist={}", closest_i, closest_d);
        }

        closest_i = index;
        closest_d = dist;
    }
    println!("i={}, dist={}", closest_i, closest_d);
}

fn run2() {

    let mut particles = read_particles();
    println!("initial count is {}", particles.len());

    for _ in 0..1000 {

        let mut counts: HashMap<Vec3, usize> = HashMap::new();
        for i in 0..particles.len() {
            let particle = &mut particles[i];
            particle.step();
        }

        for particle in particles.iter() {
            *counts.entry(particle.p).or_insert(0) += 1;
        }

        let mut survivors: Vec<Particle> = Vec::new();
        for i in 0..particles.len() {
            let particle = particles[i];
            if counts[&particle.p] == 1 {
                survivors.push(particle);
            }
        }
        particles = survivors;
    }

    println!("survivor count is {}", particles.len());
}
