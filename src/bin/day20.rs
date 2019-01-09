#[macro_use]
extern crate nom;

use nom::{digit};

const INPUT: &'static str = include_str!("../../input/Day20.txt");

fn main() {
    day1();
}

fn day1() {
    run1();
    run2();
}

#[derive(Debug)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

named!(
    integer<&str, i32>, flat_map!(
    recognize!(
        tuple!(
            opt!(char!('-')),
            digit
    )),
    parse_to!(i32)
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

fn run1() {
    let mut particles: Vec<Particle> = Vec::new();
    for line in INPUT.lines() {
        let particle = particle(line.trim());
        println!("{:?}", particle);
        particles.push(particle.unwrap().1);
    }
}

fn run2() {
}
