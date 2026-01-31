/*
 * Here is a comment
 */
fn main(){
    let name = String::from("Dustin");
    let greeting = format!("Hello {0}", name);
    let age: u128 = 48;
    let out = format!("{greeting}! You are {age} years old... damn {name}... that's old!", greeting=greeting, name=name, age=age);
    println!("{}", out);

    println!("{}", age);
    println!("{:b}", age);
    println!("{:o}", age);
    println!("{:x}", age);

    println!("{:>5}", age);
    println!("{:0>5}", age);
    println!("{:0<5}", age);

    println!("{:0>10} {}", age, e);
}