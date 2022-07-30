use log::warn;

use re0_pest::{
    ast::{ASTKind, ASTNode, DeclareStatement},
    ParseContext,
};

use crate::{
    world::{event::Event, WorldTemplate},
    Result,
};

impl WorldTemplate {
    pub fn parse(&mut self, input: &str) -> Result<()> {
        let mut state = ParseContext::default();
        let ast = state.parse(input)?;
        self.parse_root(ast)?;
        Ok(())
    }
    fn parse_root(&mut self, input: ASTNode) -> Result<()> {
        match input.kind {
            ASTKind::Root(v) => {
                for node in v {
                    self.parse_declare(node);
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn parse_declare(&mut self, input: DeclareStatement) {
        match input.keyword.as_str() {
            "事件" | "因果" => {
                let event = match Event::parse(input) {
                    Ok(o) => o,
                    Err(_) => {
                        warn!("事件解析失败");
                        return Ok(());
                    }
                };
                if let Some(s) = self.events.insert(event.name.clone(), event) {
                    warn!("重复事件 {} 已被覆盖", s.name)
                }
            }
            _ => unreachable!("{}", input.keyword.as_str()),
        }
    }
}

impl Event {
    fn parse(event: DeclareStatement) -> Result<Event> {
        Ok(())
    }
}
