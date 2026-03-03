use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let a = match String::from("5").parse::<i32>() { Ok(a) => a, Err(p) => return Err(p), };
    let b  = match String::from("6").parse::<i32>() { Ok(b) => b, Err(p) => return Err(p) };

    println!("{} = {} = {}", a, b, a - b);

    let y = String::from("56").parse::<i32>().and_then(|y| {
        String::from("75").parse::<i32>().map(|y2| y2 * y)
    });
    println!("{:?}", y);

    let d = String::from("76").parse::<i32>()?;
    let e = String::from("23").parse::<i32>()?;
    println!("{} * {} = {}", d, e, d * e);

    let f = try!(String::from("76").parse::<i32>());
    let g = try!(String::from("6").parse::<i32>());
    println!("{} / {} = {}", f, g, f / g);

    return Ok(());
}