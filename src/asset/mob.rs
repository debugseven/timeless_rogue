use crate::asset::{Drop, Mob};
use ron::de::from_str;
use rand::{thread_rng, Rng};

impl Mob {
    pub fn add_health(&mut self, val: i32) {
        self.health += val;
    }

    pub fn drop(&self) -> Drop {
        self.drop.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn attack_roll(&self) -> i32 {
        let mut rng = thread_rng();
        rng.gen_range(self.min_attack..self.max_attack+1)
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }
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
            },
            min_attack: 0,
            max_attack: 0,
        }
    }
}

impl Drop {
    
}