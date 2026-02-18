
#[cfg(panic = "abort")]
fn abort() {
    println!("aborted");
}

#[cfg(panic = "unwind")]
fn abort() {
    println!("unwinding");
}

fn should_panic(val: String) {
    if val.contains("panicked") {
        abort();
    } else {
        println!("{}", val);
    }
}

fn main() {
    should_panic("test".to_string());
    should_panic("panicked".to_string());
    should_panic("never prints".to_string());
}