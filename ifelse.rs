
fn main() {
    let immortal = true;

    let age =
        if  immortal {
            std::u32::MAX
        } else {
            23
        };

    println!("{:#?}", age);
}