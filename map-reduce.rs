use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use std::thread;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let p = Path::new("data/nums.txt");
    let f  = match File::open(&p) { Ok(f) => f, Err(err) => panic!("couldn't open {}: {}", p.display(), err) };

    let mut r = BufReader::new(f);
    let mut line = String::new();

    let mut ts = vec![];
    while r.read_line(&mut line)? > 0 {
        let value = line.clone().trim().to_string();
        ts.push(
            thread::spawn(move || -> u32 {
                //println!("{}", value);
                let c = value.chars().map(|c| c.to_digit(10).expect("whoops")).sum::<u32>();
                c
            })
        );
        line.clear();
    }

    let c = ts.into_iter().map(|t| t.join().unwrap()).sum::<u32>();
    println!("{:?}", c);

    Ok(())
}