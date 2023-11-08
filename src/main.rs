use setup::setup;

mod setup;
mod converter;

fn main() {
    // setup();
    converter::convert("test.txt".to_string());
    println!("Setup complete")
}
