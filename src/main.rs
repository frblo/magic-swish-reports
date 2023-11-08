// use setup::setup;

mod setup;
mod converter;

fn main() {
    // setup();
    converter::convert("rawdata.txt".to_string());
    println!("Setup complete")
}
