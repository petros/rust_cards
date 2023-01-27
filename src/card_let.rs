pub fn execute() {
    println!("--- let vs match ---");
    let optional = Some(10);
    if let Some(i) = optional {
        println!("Matches and it is {i}");
    } else {
        println!("Doesn't match!");
    }

    // This is equivalent to
    match optional {
        Some(i) => {
            println!("Matches and it is {i}");
        }
        _ => {
            println!("Doesn't match!");
        }
    }
    println!();
}
