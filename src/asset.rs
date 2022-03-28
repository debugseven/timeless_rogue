use ron::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
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
        }
    }
}

// -----------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct Drop {
    pub items: Vec<String>,
    pub gold: u32,
}

#[derive(Debug, Deserialize)]
pub struct Mob {
    pub name: String,
    pub health: u32,
    pub drop: Drop,
}

impl From<&str> for Mob {
    fn from(data: &str) -> Self {
        match from_str(data) {
            Ok(mob) => mob,
            Err(e) => {
                println!("Failed to parse mob '{}': {}", e, data);
                std::process::exit(1);
            }
        }
    }
}

impl Default for Mob {
    fn default() -> Self {
        Self {
            name: "no_name".to_string(),
            health: 0,
            drop: Drop {
                items: vec![],
                gold: 0,
            }
        }
    }
}