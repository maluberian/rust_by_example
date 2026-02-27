
struct HolyDog {
    cow: Option<HolyCow>
}
impl HolyDog {
    fn unwrap(&self) -> Option<i32> {
        self.cow?.i
    }
}

#[derive(Clone, Copy)]
struct HolyCow {
    i: Option<i32>
}

fn main() {
   let val1 =  HolyDog{ cow: Some(HolyCow{i: Some(5)})};
   let val2 =  HolyDog{ cow: Some(HolyCow{i: None})};

    println!("{}", val1.unwrap().expect("Ooops"));
    println!("{}", match val2.unwrap() { Some(x) => x, None => -1 });
}