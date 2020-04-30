extern crate cssparser;
use cssparser::{Parser, ParserInput, Token};

/// CSSを展開して、SelectorごとのStyle指定に置き換えた内容を保持します
#[derive(Debug, Clone, PartialEq)]
pub struct Style<'a> {
    pub block_assigns: Vec<BlockAssign<'a>>,
}

/// 1selectorに対するStyle指定を示す
/// h1 { /* ........ */ }
/// ^    ^^^^^^^^^^^^^^
/// |                 |
/// selector          assigns
#[derive(Debug, Clone, PartialEq)]
pub struct BlockAssign<'a> {
    pub selectors: Vec<Token<'a>>,
    pub assigns: Vec<Assign<'a>>,
}

impl<'a> BlockAssign<'a> {
    pub fn new() -> Self {
        BlockAssign {
            selectors: Vec::new(),
            assigns: Vec::new(),
        }
    }
}

/// 1Style指定を示す
/// border: solid 2px green
/// ^       ^^^^^^^^^^^^^^^
/// |       |
/// key     values
#[derive(Debug, Clone, PartialEq)]
pub struct Assign<'a> {
    pub keys: Vec<Token<'a>>, // 多分要素1にしかならない
    pub values: Vec<Token<'a>>,
}

impl<'a> Assign<'a> {
    pub fn new() -> Self {
        Assign {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<'a> Style<'a> {
    fn parse_assign(parser: &mut Parser) -> Option<Assign<'a>> {
        let mut assign = Assign::new();

        if assign.keys.len() > 0 { Some(assign) } else { None }
    }

    /// CSSのSelector含む1Blockを解析して返します
    /// 解析に失敗した場合、終端に達した場合はNoneが返ります
    fn parse_block(parser: &mut Parser<'a, 'a>) -> Option<BlockAssign<'a>> {
        let mut block_assign: BlockAssign<'a> = BlockAssign::new();

        while let Ok(token) = parser.next() {
            match token {
                Token::ParenthesisBlock | Token::CurlyBracketBlock | Token::SquareBracketBlock => {
                    debug_assert!(block_assign.selectors.len() > 0); // selectorは存在するはず
                    // 子要素を解析する
                    // let nested: Result<(), cssparser::ParseError<'_, ()>>  =
                    //     parser.parse_nested_block(|p: &mut Parser| Style::parse(p)); // TODO: error typeをまともにする
                },
                Token::Function(name) => {
                    println!("{:?}({})", token, name);
                    debug_assert!(false); // selectorにfunctionは存在しないはず...
                }
                // selector要素をすべて連結しとく
                _ => {
                    block_assign.selectors.push(token.clone()); // TODO: cloneするほどかは微妙
                },
            }
        }

        if block_assign.selectors.len() > 0 { Some(block_assign) } else { None }
    }
    pub fn new(css_text: &'a str) -> Self {
        let mut dst = Style {
            block_assigns: Vec::new(),
        };
        let mut parser_in = ParserInput::new(css_text);
        let mut parser = Parser::new(&mut parser_in);

        // 先頭から順番に解析するだけ
        while let Some(block) = Style::parse_block(&mut parser) {
            dst.block_assigns.push(block);
        }

        dst
    }
}

#[cfg(test)]
mod tests {
    use super::Style;

    #[test]
    fn test_style_new() {
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
        let style = Style::new(css_text);
        println!("#Style\n{:?}", style);
    }
}
