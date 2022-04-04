use serde::Deserialize;

pub mod mob;
pub mod item;
pub mod dungeon;

#[derive(Debug, Deserialize, Clone)]
pub enum Category {
    None,
    Effect,
    Attack,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    name: String,
    cat: Category,
    health: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Drop {
    pub items: Vec<(u32, String)>,
    pub gold: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mob {
    name: String,
    health: i32,
    drop: Drop,
    min_attack: i32,
    max_attack: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dungeon {
    name: String,
    stages: Vec<String>,
    #[serde(skip)]
    curr: usize,
}