use super::event::Event;
use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) struct Marker {
    pos: usize,
}

impl Marker {
    pub(super) fn new(pos: usize) -> Self {
        Self { pos }
    }

    pub(super) fn complete(self, p: &mut Parser, kind: SyntaxKind) {
        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode { kind };
        p.events.push(Event::FinishNode);
    }
}
