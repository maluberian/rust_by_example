use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    println!("path: {}", args[0]);
    println!("args: {:?}", &args[1..]);
}