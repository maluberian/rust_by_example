use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let a = match String::from("5").parse::<i32>() { Ok(a) => a, Err(p) => return Err(p), };
    let b  = match String::from("6").parse::<i32>() { Ok(b) => b, Err(p) => return Err(p) };

    println!("{} = {} = {}", a, b, a - b);

    return Ok(());
}