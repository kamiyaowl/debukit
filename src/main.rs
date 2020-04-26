use debukit::communicator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_url = "https://kamiyaowl.github.io/";// "http://histo.io/";
    let html_text = communicator::get_blocking(test_url)?;
    println!("#html: {}", html_text);

    Ok(())
}
