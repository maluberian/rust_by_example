
use std::mem;

#[derive(Debug, Clone)]
enum Sex { MALE, FEMALE, OTHER }

#[derive(Debug, Clone)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}
#[derive(Debug,Clone)]
struct Human {
    name: String,
    weight: i32,
    sex: Sex,
}

fn main() {
    let add = Address { street: "London".to_string(), city: "London".to_string(), state: "MI".to_string(), zip: "49546".to_string() };
    let me = Human { name: "Dustin Clifford".to_string(), weight: 2000, sex: Sex::MALE };
    let boxed_add = Box::new(add.clone());
    let boxed_me = Box::new(me.clone());

    println!("me: {:#?}", mem::size_of_val(&me));
    println!("add: {:#?}", mem::size_of_val(&add));
    println!("boxed_add: {:#?}", mem::size_of_val(&boxed_add));

    let unboxed_me = *boxed_me;
    println!("unboxed_me: {:#?}", mem::size_of_val(&unboxed_me));

    let unboxed_add = *boxed_add;
    println!("unboxed_add: {:#?}", mem::size_of_val(&unboxed_add));

}