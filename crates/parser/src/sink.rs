use super::event::Event;
use crate::{parser::ParseError, Parse};
use lexer::Token;
use rowan::{GreenNodeBuilder, Language};
use std::mem;
use syntax::{FelixFlowLanguage, SyntaxKind};

pub(crate) struct Sink<'t, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'t [Token<'input>],
    cursor: usize,
    events: Vec<Event>,
    errors: Vec<ParseError>,
}

impl<'t, 'input> Sink<'t, 'input> {
    pub(crate) fn new(tokens: &'t [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            errors: Vec::new(),
        }
    }

    pub(crate) fn finish(mut self) -> Parse {
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    foward_parent,
                } => {
                    let mut kinds = vec![kind];

                    let mut idx = idx;
                    let mut foward_parent = foward_parent;

                    while let Some(fp) = foward_parent {
                        idx += fp;

                        foward_parent = if let Event::StartNode {
                            kind,
                            foward_parent,
                        } =
                            mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            foward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder
                            .start_node(FelixFlowLanguage::kind_to_raw(kind));
                    }
                }
                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => {}
                Event::Error(error) => self.errors.push(error),
            }

            self.eat_trivia();
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];
        self.builder
            .token(FelixFlowLanguage::kind_to_raw(kind.into()), text.into());
        self.cursor += 1;
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !SyntaxKind::from(token.kind).is_trivia() {
                break;
            }

            self.token()
        }
    }
}
