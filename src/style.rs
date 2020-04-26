extern crate scraper;
use scraper::Selector;

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    body: Selector
}

impl Style {
    pub fn parse_all(css_text: &str) -> Option<Self> {
        let body = Selector::parse(css_text);
        if body.is_ok() {
            Some(Style {
                body: body.unwrap(),
            })
        } else {
            println!("{:?}", body);
            debug_assert!(false); // TODO: エラーハンドリングは考えたい
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Style;

    #[test]
    fn test_parse_all() {
        let css_text = r#"
            p {
                background-color: red;
                color: #c5c5c5;
                border: solid 2px green;
            }
        "#;
        let result = Style::parse_all(css_text); // TODO: Noneが帰ってくる
        println!("#Style\n{:?}", result);
    }
}