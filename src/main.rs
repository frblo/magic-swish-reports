mod setup;
mod converter;

fn main() {
    setup::setup();
    // converter::convert("rawdata.txt".to_string());
    println!("Setup complete")
}
