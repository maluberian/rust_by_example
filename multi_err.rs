use std::error;
use std::num::ParseIntError;

use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
enum CalcError {
    EmptyVec,
    BunkError(ParseIntError),
}
impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalcError::EmptyVec => write!(f, "Empty vector"),
            CalcError::BunkError(..) => write!(f, "Bunk error"),
        }
    }
}
impl error::Error for CalcError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CalcError::EmptyVec => None,
            CalcError::BunkError(ref err) => Some(err),
        }
    }
}


fn parse_to_int32_and_sub1(vec: Vec<&str>) -> Result<i32> {
    let first = vec.get(0).ok_or(CalcError::EmptyVec)?;
    let i = first.parse::<i32>().map_err(|e| CalcError::BunkError(ParseIntError::from(e)))?;

    Ok(i)
}

fn print(r: Result<i32>) {
    match r {
        Ok(i) => println!("The number is \"{}\"", i),
        Err(e) => println!("Error: \"{}\"", e),
    }
}

fn main() {
    let v1 = vec!["53", "22", "2"];
    let v2 = vec![];
    let v3 = vec!["b53"];

    print(parse_to_int32_and_sub1(v1));
    print(parse_to_int32_and_sub1(v2));
    print(parse_to_int32_and_sub1(v3));
}