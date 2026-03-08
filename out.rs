use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    let d =
        "
TEST ME
THIS IS SOME DATA
BOOPY BOOPY
    ";
    let p = Path::new("data/out.txt");
    let mut f = match File::create(&p) {
        Ok(f) => f,
        Err(err) => panic!("couldn't open {}: {}", p.display(), err)
    };

    match f.write_all(d.as_bytes()) {
        Ok(_) => println!("wrote data to {}", p.display()),
        Err(err) => panic!("couldn't write to {}: {}", p.display(), err)
    }

}