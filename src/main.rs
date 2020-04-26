use debukit::communicator;

fn main() -> Result<(), Box<dyn std::error::Error> {
    let test_url = "https://kamiyaowl.github.io/";
    let htmlText = communicator::get_blocking(test_url);
}
