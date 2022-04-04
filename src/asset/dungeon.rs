use crate::asset::Dungeon;
use ron::de::from_str;

impl Dungeon {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Iterator for Dungeon {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stages.len() - 1 < self.curr {
            return None;
        }

        let mob = self.stages[self.curr].clone();
        self.curr += 1;
        Some(mob)
    }
}

impl From<&str> for Dungeon {
    fn from(data: &str) -> Self {
        match from_str(data) {
            Ok(dungeon) => dungeon,
            Err(e) => {
                println!("Failed to parse dungeon: {}", e);
                std::process::exit(1);
            }
        }
    }
}

impl Default for Dungeon {
    fn default() -> Self {
        Self {
            name: "no_name".to_string(),
            stages: vec![],
            curr: 0,
        }
    }
}