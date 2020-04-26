extern crate cssparser;
use cssparser::{Parser, ParserInput};

#[derive(Debug, Clone, PartialEq)]
pub struct Style {}

impl Style {
    pub fn parse_all(css_text: &str) -> Self {
        let mut dst = Style {};

        let mut parser_in = ParserInput::new(css_text);
        let mut parser = Parser::new(&mut parser_in);

        // TODO: CSS Selectorと属性の紐付け
        // Ident, Delimを束ねてSelectorを構成する
        // CurlyBacketBlockを展開して子要素としてまとめる
        loop {
            let hoge = parser.next();
            println!("{:?}", hoge);
            if hoge.is_err() {
                break;
            }
        }

        dst
    }
}

#[cfg(test)]
mod tests {
    use super::Style;

    #[test]
    fn test_style_parse_all() {
        let css_text = r#"
            h1 {
                scale: 200%;
            }
            p {
                background-color: red;
                color: #c5c5c5;
                border: solid 2px green;
            }
            div div div {
                display: hidden;
            }
            div + p {
                color: #aa9955;
            }
            table ~ p {
                background-color: green;
            }
        "#;
        let result = Style::parse_all(css_text);
        println!("#Style\n{:?}", result);
    }
}
