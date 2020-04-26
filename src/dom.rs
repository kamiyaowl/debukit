extern crate scraper;
use scraper::Html;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dom {
    pub body: Html, // TODO: scraper非依存にするなら自作の構造で置き換え
}

impl Dom {
    pub fn parse_all(html_text: &str) -> Self {
        let document = Html::parse_document(html_text);
        Dom {
            body: document,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dom;
    #[test]
    fn test_parse_all() {
        let result = Dom::parse_all("<!doctype html><html><head><title>test_title</title></head><body><h1>hello</h1></body></html>");
        println!("{:?}", result)
    }
}
