use std::fmt::Display;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    #[regex("[ \n]+")]
    Whitespace,

    #[token("fn")]
    FnKw,

    #[token("let")]
    LetKw,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex("[0-9]+")]
    Number,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex("#.*")]
    Comment,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TokenKind::Whitespace => "whitespace",
            TokenKind::FnKw => "'fn'",
            TokenKind::LetKw => "'let'",
            TokenKind::Ident => "identifier",
            TokenKind::Number => "number",
            TokenKind::Plus => "'+'",
            TokenKind::Minus => "'-'",
            TokenKind::Star => "'*'",
            TokenKind::Slash => "'/'",
            TokenKind::Equals => "'='",
            TokenKind::LParen => "'('",
            TokenKind::RParen => "')'",
            TokenKind::LBrace => "'{'",
            TokenKind::RBrace => "'}'",
            TokenKind::Comment => "comment",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);
        let token = lexer.next().unwrap();

        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[test]
    fn lex_spaces() {
        check("  ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_space_and_newlines() {
        check("  \n", TokenKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", TokenKind::FnKw);
    }

    #[test]
    fn lex_let_keyword() {
        check("let", TokenKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_ident() {
        check("abcd", TokenKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_ident() {
        check("abcd123abc456", TokenKind::Ident);
    }
    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", TokenKind::Ident);
    }

    #[test]
    fn lex_number() {
        check("123456", TokenKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", TokenKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check("{", TokenKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", TokenKind::RBrace);
    }

    #[test]
    fn lex_left_parenteses() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_parenteses() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_comment() {
        check("# foo", TokenKind::Comment);
    }
}
