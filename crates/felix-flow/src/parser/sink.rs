use super::event::Event;
use crate::syntax::FelixFlowLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub(super) struct Sink {
    builder: GreenNodeBuilder<'static>,
    events: Vec<Event>,
}

impl Sink {
    pub(super) fn new(events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        let mut reordered_events = self.events.clone();

        for (id, event) in self.events.into_iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(id);
                reordered_events.insert(checkpoint, Event::StartNode { kind })
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => self
                    .builder
                    .start_node(FelixFlowLanguage::kind_to_raw(kind)),
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => self
                    .builder
                    .token(FelixFlowLanguage::kind_to_raw(kind), text.as_str()),
                Event::FinishNode => self.builder.finish_node(),
            }
        }

        self.builder.finish()
    }
}
