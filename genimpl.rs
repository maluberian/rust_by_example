use std::any::type_name;
use std::fmt::{Display, Debug};

trait Printable {
    fn print_value(&self);
}
struct GenVal<T: Display> {
    value: T
}
impl <T: Display> GenVal<T> {
    fn get_value(self) -> T {
       self.value
    }
}
impl <T> Printable for GenVal<T>  where T: Display {
    fn print_value(&self) {
        println!("{} : {}", type_name::<T>(), self.value);
    }
}

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    let i = GenVal { value: 5 };
    let j = GenVal { value: "fire" };
    let i2 = GenVal { value: 5 };
    let j2 = GenVal { value: "fire" };

    i.print_value();
    j.print_value();
}