extern crate cssparser;
use cssparser::{Parser, ParserInput, Token};

/// Styleseet適用優先順位を示します
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CascadeOrder {
    /// ブラウザの宣言、一番優先度が低い
    Browser,
    /// ユーザーの宣言
    /// 重要な宣言であればtrue、この場合製作者の内容よりも順位が高い
    User(bool),
    /// 製作者の宣言
    /// 重要な宣言であればtrue
    Maker(bool),
}

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
    pub order: CascadeOrder,
    pub selectors: Vec<Token<'a>>,
    pub assigns: Vec<Assign<'a>>,
}

impl<'a> BlockAssign<'a> {
    pub fn new() -> Self {
        BlockAssign {
            order: CascadeOrder::Browser, // デフォルトでは優先順序最低
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
    pub key: Token<'a>,
    pub values: Vec<Token<'a>>,
}

impl<'a> Assign<'a> {
    pub fn new() -> Self {
        Assign {
            key: Token::Colon, // ありえない値に設定
            values: Vec::new(),
        }
    }
}

impl<'a> Style<'a> {
    /// テキストからStyle指定に変換します
    pub fn new(css_text: &'a str, order: CascadeOrder) -> Self {
        let mut dst = Style {
            block_assigns: Vec::new(),
        };
        let mut parser_in: ParserInput = ParserInput::new(css_text);
        let mut parser: Parser = Parser::new(&mut parser_in);

        // 先頭から順番に解析するだけ
        let mut index = 0;

        while let Ok(token) = parser.next() {
            match token {
                Token::ParenthesisBlock | Token::CurlyBracketBlock | Token::SquareBracketBlock => {
                    debug_assert!(!dst.block_assigns.is_empty()); // selectorが事前に一つ存在したはず
                    debug_assert!(!dst.block_assigns[0].selectors.is_empty()); // selectorは存在するはず

                    // 子要素を解析する
                    let assigns_result: Result<Vec<Assign<'a>>, cssparser::ParseError<'_, ()>> =
                        parser.parse_nested_block(|p: &mut Parser<'a, '_>| {
                            let mut assigns: Vec<Assign<'a>> = Vec::new();
                            while let Some(assign) = Style::parse_assign(p) {
                                assigns.push(assign);
                            }
                            Ok(assigns)
                        });
                    // block_assignを完成させる
                    if let Ok(assigns) = assigns_result {
                        dst.block_assigns[index].assigns = assigns;
                    } else {
                        println!(
                            "[Error] Parser Error in parse_nested_block.{:?}",
                            assigns_result
                        );
                        debug_assert!(false); // 基本は子要素もParseできるはず...
                    }
                    index += 1;
                }
                Token::Function(name) => {
                    println!(
                        "[Error] Function should not exist in selector.{:?}({})",
                        token, name
                    );
                    debug_assert!(false); // selectorにfunctionは存在しないはず...
                }
                // selector要素をすべて連結しとく
                _ => {
                    if index >= dst.block_assigns.len() {
                        dst.block_assigns.push(BlockAssign::new());
                    }
                    dst.block_assigns[index].selectors.push(token.clone()); // TODO: cloneで良いんだっけ, parse_assignも同様
                    dst.block_assigns[index].order = order;
                }
            }
        }

        dst
    }

    /// 他のStyle指定を取り込みます
    pub fn concat(self: &mut Style<'a>, arg: &Style<'a>) {
        for block_assign in &arg.block_assigns {
            self.block_assigns.push(block_assign.clone());
        }
    }

    /// BlockAssign指定ごと取り込みます、これはHTML上でのStyle属性等で指定された内容のマージに使用する想定です
    pub fn add(self: &mut Style<'a>, block_assign: &BlockAssign<'a>) {
        self.block_assigns.push(block_assign.clone());
    }

    /// 特異性に基づき、block_assignを並び替えます
    /// これはパフォーマンス上の対策で、実際に適用する際はCascadeOrderに従って適用する必要があります
    pub fn sort_from_specificity(self: &mut Style<'a>) {
        // とりあえずselectorが多い順にしてある
        self.block_assigns
            .sort_by(|a: &BlockAssign<'a>, b: &BlockAssign<'a>| {
                a.assigns.len().cmp(&b.assigns.len())
            });
    }

    fn parse_assign(parser: &mut Parser<'a, '_>) -> Option<Assign<'a>> {
        let mut is_set_key = false; // keyに値をセットしたらtrue
        let mut assign: Assign<'a> = Assign::new();

        // Ident, Colon, Values, Semicolonの順序で分解
        while let Ok(token) = parser.next() {
            match token {
                Token::Ident(_) => {
                    if !is_set_key {
                        assign.key = token.clone();
                        is_set_key = true;
                    } else {
                        // Tokenで方統一したいのでそのままぶっこんどく
                        assign.values.push(token.clone());
                    }
                }
                Token::Semicolon => {
                    // 1要素の指定分は見終わったのでreturnする
                    debug_assert!(is_set_key);
                    return Some(assign);
                }
                // 要素ではないものは弾く
                Token::Colon
                | Token::Comma
                | Token::IncludeMatch
                | Token::DashMatch
                | Token::PrefixMatch
                | Token::SuffixMatch
                | Token::CDO
                | Token::CDC
                | Token::ParenthesisBlock
                | Token::SquareBracketBlock
                | Token::CurlyBracketBlock
                | Token::CloseParenthesis
                | Token::CloseSquareBracket
                | Token::CloseCurlyBracket => {}
                // 要素をすべて追加しておく
                _ => {
                    assign.values.push(token.clone());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::CascadeOrder;
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
            .canvas  {
                z-index: 100;
            }
        "#;
        let style = Style::new(css_text, CascadeOrder::Browser);
        println!("#Style\n{:#?}", style);
    }
}
