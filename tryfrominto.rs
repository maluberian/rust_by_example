use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct OddNumber(i32);

impl TryFrom<i32> for OddNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 1 {
            Ok(OddNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(OddNumber::try_from(9), Ok(OddNumber(9)));
    assert_eq!(OddNumber::try_from(4), Err(()));

    // TryInto

    let result: Result<OddNumber, ()> = 9i32.try_into();
    assert_eq!(result, Ok(OddNumber(9)));
    let result: Result<OddNumber, ()> = 4i32.try_into();
    assert_eq!(result, Err(()));
}