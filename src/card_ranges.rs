pub fn execute() {
    println!("--- Ranges ---");

    let say = "My flowers are beautiful";

    // RangeTo
    println!("{}", &say[..1]);
    //=> M

    // RangeToInclusive
    println!("{}", &say[..=1]);
    //=> My

    // RangeFull
    println!("{}", &say[..]);
    //=> My flowers are beautiful

    // Range
    println!("{}", &say[3..10]);
    //=> flowers

    // RangeFrom
    println!("{}", &say[11..]);
    //=> are beautiful

    println!();
}
