pub mod macros;
pub mod asset;

use crate::asset::{Item, Mob};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Data {
    pub items: HashMap<String, Item>,
    pub enemies: HashMap<String, Mob>
}

impl Data {
    pub fn new() -> Self {
        Self {
            items: item!["beer", "fire_rune"],
            enemies: enemy!["crow", "phoenix"],
        }
    }
}

