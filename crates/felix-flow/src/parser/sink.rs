use super::event::Event;
use crate::lexer::{Lexeme, SyntaxKind};
use crate::syntax::FelixFlowLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use smol_str::SmolStr;

pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            cursor: 0,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        let mut reordered_events = self.events.clone();

        for (id, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(id);
                reordered_events.insert(*checkpoint, Event::StartNode { kind: *kind })
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => self
                    .builder
                    .start_node(FelixFlowLanguage::kind_to_raw(kind)),
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
            }

            self.eat_whitespace();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: SyntaxKind, text: SmolStr) {
        self.builder
            .token(FelixFlowLanguage::kind_to_raw(kind), text.as_str());
        self.cursor += 1;
    }

    fn eat_whitespace(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if lexeme.kind != SyntaxKind::Whitespace {
                break;
            }

            self.token(lexeme.kind, lexeme.text.into())
        }
    }
}
