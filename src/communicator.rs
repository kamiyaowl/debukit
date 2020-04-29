extern crate reqwest;

/// http/https通信関係全般を管理します、これには通信時のCache管理も含みます
pub struct Communicator {
    // TODO: cacheの実装
}

impl Communicator {
    pub fn get_blocking<T: reqwest::IntoUrl>(url: T) -> Result<String, Box<dyn std::error::Error>> {
        // TODO: Cache-Control, Expires, Last-Modifiedを参照してローカルキャッシュする
        let res: reqwest::blocking::Response = reqwest::blocking::get(url)?;
        let text: String = res.text()?;

        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::Communicator;
 
    #[test]
    fn test_html_get_blocking() -> Result<(), Box<dyn std::error::Error>> {
        let text = Communicator::get_blocking("https://kamiyaowl.github.io/")?;
        println!("{}", text);
        Ok(())
    }
}