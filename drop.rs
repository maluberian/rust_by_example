
struct Dropper {
    value: String,
}
impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Droppy drop drop of {}", self.value);
    }
}

fn take_control(c: &Dropper) {
    println!("Take control of {}", c.value);
}

fn main() {
    let thing = Dropper{ value: String::from("hello") };

    take_control(&thing);

    println!("{} dropped? (level1) == nope", thing.value);
}