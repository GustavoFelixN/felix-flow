#[macro_use]
extern crate num_derive;

use lexer::TokenKind;
use num_traits::{FromPrimitive, ToPrimitive};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    Whitespace,
    FnKw,
    LetKw,
    Ident,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comment,
    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableDef,
    VariableRef,
    Error,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::FnKw => Self::FnKw,
            TokenKind::LetKw => Self::LetKw,
            TokenKind::Ident => Self::Ident,
            TokenKind::Number => Self::Number,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::Comment => Self::Comment,
        }
    }
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SyntaxKind::Whitespace => "whitespace",
            SyntaxKind::FnKw => "'fn'",
            SyntaxKind::LetKw => "'let'",
            SyntaxKind::Ident => "identifier",
            SyntaxKind::Number => "number",
            SyntaxKind::Plus => "'+'",
            SyntaxKind::Minus => "'-'",
            SyntaxKind::Star => "'*'",
            SyntaxKind::Slash => "'/'",
            SyntaxKind::Equals => "'='",
            SyntaxKind::LParen => "'('",
            SyntaxKind::RParen => "')'",
            SyntaxKind::LBrace => "'{'",
            SyntaxKind::RBrace => "'}'",
            SyntaxKind::Comment => "comment",
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FelixFlowLanguage {}

impl rowan::Language for FelixFlowLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<FelixFlowLanguage>;
