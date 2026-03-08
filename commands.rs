use std::process::{Command, Stdio};
use std::io::prelude::*;

fn main() {
    let output = Command::new("python3").arg("--version").output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        println!("Error!!!\n{}", String::from_utf8_lossy(&output.stderr));
    }

    let input_string =
        "
How do I love thee? Let me count the ways.
        ";
    let mut cmd = Command::new("wc");
    let process = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(process) => process,
        Err(e) => panic!("failed to spawn wc: {}", e),
    };

    match process.stdin.unwrap().write_all(input_string.as_bytes()) {
        Ok(_) => println!("data sent to wc"),
        Err(e) => println!("failed to send data : {}", e),
    }

    let mut o = String::new();
    match process.stdout.unwrap().read_to_string(&mut o) {
        Ok(_) => println!("wc responded with: {}", o),
        Err(e) => println!("failed to read wc responded with: {}", e),
    }

    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of main");
}
