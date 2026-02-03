use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {} centered at ({}, {})", self.radius, self.x, self.y)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(x) => Ok(Circle {x: 0, y: 0, radius: x}),
            Err(e) => Err(e)
        }
    }
}

fn main() {
    let circle = Circle { x: 1, y: 34, radius: 2};

    println!("{}", circle.to_string());

    let c: Circle = "   1234  ".parse().unwrap();
    println!("{}", c);
}