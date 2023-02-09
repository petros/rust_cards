mod card_if_let;
mod card_ranges;

fn main() {
    println!("This is cards!");
    card_ranges::execute();
    card_if_let::execute();
}
