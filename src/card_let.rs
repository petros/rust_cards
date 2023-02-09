pub fn execute() {
    println!("--- let vs match ---");
    let optional = Some(10);

    // The if let way
    if let Some(i) = optional {
        println!("Matches and it is {i}");
    } else {
        println!("Doesn't match!");
    }

    // The match way
    match optional {
        Some(i) => println!("Matches and it is {i}"),
        _ => println!("Doesn't match!"),
    }

    println!();
}
