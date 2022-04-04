use crate::asset::{Category, Item};
use ron::de::from_str;

impl Item {
    pub fn health(&self) -> i32 {
        self.health
    }
}

impl From<&str> for Item {
    fn from(data: &str) -> Self {
        match from_str(data) {
            Ok(item) => item,
            Err(e) => {
                println!("Failed to parse item: {}", e);
                std::process::exit(1);
            }
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: "no_name".to_string(),
            cat: Category::None,
            health: 0,
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}