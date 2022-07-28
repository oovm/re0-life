mod evaluate;
use re0_pest::ast::ASTKind;
use std::collections::BTreeMap;

use crate::{world::World, Result};

pub struct GameVM {
    pub templates: Template,
    pub world: World,
    pub experience: BTreeMap<usize, String>,
    pub item: BTreeMap<usize, String>,
}

pub struct Template {
    pub words: Vec<World>,
    pub items: Vec<Template>,
    pub events: Vec<Template>,
    pub functions: BTreeMap<String, Re0Function>,
}

pub type Re0Function = fn(&mut GameVM, &[ASTKind]) -> Result<ASTKind>;

impl GameVM {
    pub fn register_function(&mut self, name: &str, func: Re0Function) {
        todo!()
        // self.world.regiest_function(name, func);
    }
}
