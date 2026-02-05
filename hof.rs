fn is_me(s: &str) -> bool {
    s.split(' ').any(|x| x.to_lowercase() == String::from("Dustin").to_string().to_lowercase())
}

fn main() {
    let name_vec = vec!["Dustin Clifford", "Fred Muller", "Cory"];

    name_vec.into_iter().map(|name| is_me(&name)).for_each(|n| println!("{}", n));
}