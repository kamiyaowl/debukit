extern crate reqwest;

pub fn get_blocking<T: reqwest::IntoUrl>(url: T) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Cache-Control, Expires, Last-Modifiedを参照してローカルキャッシュする
    let res: reqwest::blocking::Response = reqwest::blocking::get(url)?;
    let text: String = res.text()?;

    Ok(text)
}

#[test]
fn test_get_blocking() -> Result<(), Box<dyn std::error::Error>> {
    let text = get_blocking("https://kamiyaowl.github.io/")?;
    println!("{}", text);
    Ok(())
}
