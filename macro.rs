
macro_rules! create_print {
    ($func_name:ident,$exp:expr ) => {
        fn $func_name() {
            println!("{}", $exp);
        }
    };
}

create_print!(tst, String::from("test"));

fn main() {
    tst();
}