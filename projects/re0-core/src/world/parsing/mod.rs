use log::{error, warn};

use re0_pest::{
    ast::{ASTKind, ASTNode, DeclareStatement},
    ParseContext,
};

use crate::{
    world::{event::Event, item::Talent, WorldTemplate},
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
                    Err(e) => {
                        error!("事件解析失败: {}", e);
                        return;
                    }
                };
                if let Some(s) = self.events.insert(event.id.clone(), event) {
                    warn!("重复声明 {} 已被覆盖", s.id)
                }
            }
            "技能" => self.parse_talent(input, false, true, false),
            "命格" => self.parse_talent(input, true, true, false),
            "天赋" => self.parse_talent(input, true, true, true),
            "物品" => self.parse_talent(input, false, false, false),
            "装备" => self.parse_talent(input, false, true, false),
            "珍宝" => self.parse_talent(input, true, true, false),
            "异宝" => self.parse_talent(input, true, true, true),
            _ => error!("未知声明: {}", input.keyword),
        }
    }
    fn parse_talent(&mut self, input: DeclareStatement, initial: bool, unique: bool, keep: bool) {
        let talent = match Talent::parse(input, initial, unique, keep) {
            Some(v) => v,
            None => {
                error!("解析失败: {}", e);
                return;
            }
        };
        if let Some(s) = self.talents.insert(talent.id.clone(), talent) {
            warn!("重复声明 {} 已被覆盖", s)
        }
    }
}

impl Event {
    fn parse(event: DeclareStatement) -> Result<Event> {
        let display_name = event.get_string(&["显示"]).unwrap_or(event.symbol.as_str()).to_string();
        let description = event.get_string(&["描述"]).unwrap_or(event.comment.as_str()).to_string();
        let manually = event.keyword == "因果";
        let event = Event {
            id: event.symbol,
            display_name,
            description,
            manually,
            condition: vec![],
            age_check: (0, 0),
            effect: vec![],
        };
        Ok(event)
    }
}

impl Talent {
    fn parse(talent: DeclareStatement, initial: bool, unique: bool, keep: bool) -> Result<Talent> {
        let display_name = talent.get_string(&["显示"]).unwrap_or(talent.symbol.as_str()).to_string();
        let description = talent.get_string(&["描述"]).unwrap_or(talent.comment.as_str()).to_string();
        let talent = Talent { id: event.symbol, display_name, description, initial, rarity: 100, unique, keep };
        Ok(talent)
    }
}
