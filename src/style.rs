extern crate cssparser;
use cssparser::{Parser, ParserInput, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Style {}

impl Style {
    fn parse<'i, 't, E>(parser: &mut Parser<'i, 't>) -> Result<(), cssparser::ParseError<'i, E>> {
        while let Ok(token) = parser.next() {
            match  token {
                Token::ParenthesisBlock | Token::CurlyBracketBlock | Token::SquareBracketBlock => {
                    println!("==========");

                    // 子要素を再帰して解析する
                    let nested: Result<(), cssparser::ParseError<'_, ()>>  =
                        parser.parse_nested_block(|p: &mut Parser| Style::parse(p)); // TODO: error typeをまともにする
                    
                    println!("==========\n");
                },
                Token::Function(name) => {
                    println!("{:?}({})", token, name);
                }
                _ => {
                    println!("{:?}", token);
                },
            }
        }
        Ok(()) // TODO: 値を返す
    }
    pub fn parse_all(css_text: &str) -> Self {
        let mut dst = Style {};

        let mut parser_in = ParserInput::new(css_text);
        let mut parser = Parser::new(&mut parser_in);

        let result: Result<(), cssparser::ParseError<'_, ()>> = Style::parse(&mut parser);

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
