pub mod macros;
pub mod asset;

use crate::asset::{Item, Mob, Dungeon};
use std::collections::HashMap;

#[derive(Debug)]
pub struct InventoryItem {
    pub amount: i32,
    pub item: Item,
}

#[derive(Debug)]
pub struct Inventory {
    items: Vec<InventoryItem>,
    pub gold: i32,
}

impl Inventory {
    pub fn add(&mut self, item: &Item, amount: i32) {
        let mut found = false;

        for i in &mut self.items {
            if &i.item == item {
                i.amount += amount;
                found = true;
                break;
            }
        }

        if !found {
            self.items.push(InventoryItem{amount, item: item.clone()});
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![],
            gold: 0,
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub items: HashMap<String, Item>,
    pub mobs: HashMap<String, Mob>,
    pub dungeons: HashMap<String, Dungeon>,
    pub inventory: Inventory,
}

impl Data {
    pub fn new(items: HashMap<String, Item>, mobs: HashMap<String, Mob>, dungeons: HashMap<String, Dungeon>) -> Self {
        Self {
            items, mobs, dungeons,
            inventory: Inventory::default(),
        }
    }
}

