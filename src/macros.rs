#[macro_export]
macro_rules! item {
    ( $( $x:expr ),* ) => {
        {
            use crate::asset::Item;
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
macro_rules! enemy {
    ( $( $x:expr ),* ) => {
        {
            use crate::asset::Mob;
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