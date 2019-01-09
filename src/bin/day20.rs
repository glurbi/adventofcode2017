#[macro_use]
extern crate nom;

use nom::{digit};

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
    position<&str, Vec3>,
    do_parse!(
        tag!("p=<") >>
        x: integer >>
        tag!(",") >>
        y: integer >>
        tag!(",") >>
        z: integer >>
        tag!(">") >>
        (Vec3 { x, y, z }))
);

named!(
    velocity<&str, Vec3>,
    do_parse!(
        tag!("v=<") >>
        x: integer >>
        tag!(",") >>
        y: integer >>
        tag!(",") >>
        z: integer >>
        tag!(">") >>
        (Vec3 { x, y, z }))
);

named!(
    acceleration<&str, Vec3>,
    do_parse!(
        tag!("a=<") >>
        x: integer >>
        tag!(",") >>
        y: integer >>
        tag!(",") >>
        z: integer >>
        tag!(">") >>
        (Vec3 { x, y, z }))
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
    let res = particle("p=<-1724,-1700,5620>, v=<44,-10,-107>, a=<2,6,-9>");
    println!("{:?}",res);
}

fn run2() {
}
