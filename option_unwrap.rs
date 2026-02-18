
fn explode(val: Option<&str>) {
    match val {
        Some("kaboom") => panic!("kaboom"),
        Some(inner) => println!("{}", inner),
        None => panic!("no value"),
    }
}

fn unwrap(val: Option<&str>) {
    let inner = val.unwrap();

    if inner == "kaboom" {
        panic!("kaboom");
    }
    println!("{}", inner);
}

fn main() {
    unwrap(Some("1"));
    //unwrap(None);
    //unwrap(Some("kaboom"));

    explode(Some("tick tick tick"));
    //explode(None);
    //explode(Some("kaboom"));
}