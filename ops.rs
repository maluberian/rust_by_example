use std::ops;
use std::fmt;

struct Res {
    x: String
}

impl ops::BitOr for Res {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self{x: format!("{} {}", self.x, rhs.x) }
    }
}

impl Drop for Res {
    fn drop(&mut self) {
        println!("I can't believe you're dropping my hard work... {} is an amazing feat!", self.x);
    }
}

impl fmt::Display for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

fn main() {
    let r = Res{x: "Hello".to_string()};
    let s = Res{x: "World".to_string()};

    println!("{}", r | s);
}