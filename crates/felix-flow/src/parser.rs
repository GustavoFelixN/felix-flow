mod event;
mod expr;
mod sink;

use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::SyntaxNode;
use event::Event;
use expr::expr;
use rowan::GreenNode;
use sink::Sink;
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    events: Vec<Event>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            events: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);
        expr(&mut self);
        self.finish_node();

        let sink = Sink::new(self.events);
        Parse {
            green_node: sink.finish(),
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind });
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    fn peek(&mut self) -> Option<Result<SyntaxKind, ()>> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self
            .lexer
            .next()
            .expect("Check made early. Its suppose to exist.");
        self.events.push(Event::AddToken {
            kind: kind.expect("Check made early. Its suppose to exist."),
            text: text.into(),
        });
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint })
    }

    fn checkpoint(&self) -> usize {
        self.events.len()
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = Parser::new(input).parse();
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![r#"Root@0..0"#])
    }
}
