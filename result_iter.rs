

fn main() {
    let numbers1 = vec!["23", "ft", "31"].into_iter().map(|x| x.parse::<i32>().ok()).collect::<Vec<Option<i32>>>();
    let numbers2: Vec<_> = vec!["23", "fst", "31"].into_iter().filter_map(|x| x.parse::<i32>().ok()).collect();

    let mut errors3 = vec![];
    let numbers3: Vec<_> = vec!["23", "f", "31"].into_iter().map(|x| x.parse::<i32>()).filter_map(|r| r.map_err(|e| errors3.push(e)).ok()).collect();

    let numbers4: Result<Vec<_>, _> = vec!["23", "t", "31"].into_iter().map(|x| x.parse::<i32>()).collect();

    let (numbers5, errors5): (Vec<_>, Vec<_>) = vec!["23", "t", "31"].into_iter().map(|x| x.parse::<i32>()).partition(Result::is_ok);

    println!("The basics\n------");
    println!("{:?}", numbers1);
    println!("{:?}", numbers2);

    println!();

    println!("The errors\n------");
    println!("{:?} (errors: {:?})", numbers3, errors3);

    println!();

    println!("The failure\n------");
    println!("{:?}", numbers4);

    println!();

    println!("The both of em\n------");
    println!("{:?}", numbers5.into_iter().map(Result::unwrap).collect::<Vec<_>>());
    println!("{:?}", errors5.into_iter().map(Result::unwrap_err).collect::<Vec<_>>());
}