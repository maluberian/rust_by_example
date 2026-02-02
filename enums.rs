// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

enum PersonType {
    Good,
    Bad,
    Politician
}

enum Number {
    Zero,
    One,
    Two,
    Three,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}



fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    use WebEvent::{PageLoad, PageUnload, KeyPress, Paste, Click, };
    let page_event = PageLoad;
    match page_event {
        PageLoad => println!("page loaded"),
        PageUnload => println!("page unloaded"),
        KeyPress(c) => println!("pressed '{}'.", c),
        Paste(s) => println!("pasted \"{}\".", s),
        Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }

    use PersonType::*;
    let person_type = Good;
    match person_type {
        Good => println!("You are a VERY good boy"),
        Bad => println!("You are a bad boy"),
        Politician => println!("You are neither good nor bad... I guess"),
    }

    println!("Zero is {}",Number::Zero as i32);
    println!("One is {}", Number::One as i32);
    println!("Two is {}", Number::One as i32);
    println!("Three is {}", Number::One as i32);

    println!("Red is 0x{:06X}", Color::Red as u32);
    println!("Green is 0x{:06X}", Color::Green as u32);
    println!("Blue is 0x{:06X}", Color::Blue as u32);
}
