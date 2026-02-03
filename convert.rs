
#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug)]
struct StrangeNumber {
    value: i32,
}
impl Into<StrangeNumber> for i32 {
 fn into(self) -> StrangeNumber {
     StrangeNumber { value: self }
 }
}

fn main() {
    let num = 205i32;
    let number = Number::from(num);

    println!("{:?}", number);

    let n: StrangeNumber = num.into();
    println!("{:?}", n);
}