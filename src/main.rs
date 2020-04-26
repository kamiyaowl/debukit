use std::fs;

use debukit::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let html_text = communicator::get_blocking("https://kamiyaowl.github.io/")?;
    let html_text = fs::read_to_string("./testdata/test1.html")?;
    println!("{}", html_text);
    Ok(())
}
