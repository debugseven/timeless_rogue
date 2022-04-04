#[macro_export]
macro_rules! item {
    ( $( $x:expr ),* ) => {
        {
            use timeless_rogue::asset::Item;
            use std::collections::HashMap;
            let mut items = HashMap::new();
            $(
                let file = include_str!(concat!("../assets/item/", $x, ".ron"));
                items.insert($x.to_string(), Item::from(file));
            )*

            items
        }
    };
}

#[macro_export]
macro_rules! mob {
    ( $( $x:expr ),* ) => {
        {
            use timeless_rogue::asset::Mob;
            use std::collections::HashMap;
            let mut mobs = HashMap::new();
            $(
                let file = include_str!(concat!("../assets/mob/", $x, ".ron"));
                mobs.insert($x.to_string(), Mob::from(file));
            )*

            mobs
        }
    };
}

#[macro_export]
macro_rules! dungeon {
    ( $( $x:expr ),* ) => {
        {
            use timeless_rogue::asset::Dungeon;
            use std::collections::HashMap;
            let mut dungeons = HashMap::new();
            $(
                let file = include_str!(concat!("../assets/dungeon/", $x, ".ron"));
                dungeons.insert($x.to_string(), Dungeon::from(file));
            )*

            dungeons
        }
    };
}