
fn print_val(b: &Box<i8>) {
    println!("{}", b);
}

fn change_val_and_print(b: &mut Box<i8>) {
    *b = Box::new(3i8);
    println!("{}", b);
}

fn main() {
    let imm = Box::new(5i8);

    print_val(&imm);

    let mut m = Box::new(5i8);
    change_val_and_print(&mut m);

    print_val(&m);
}