use timeless_rogue::{Data, item, mob, dungeon};
use colored::*;
use std::{thread, time};

fn main() {
    demo();
}

fn demo() {
    let items = item!["beer", "fire_rune"];
    let mobs = mob!["crow", "phoenix"];
    let dungeons = dungeon!["bird_castle"];
    let mut data = Data::new(items, mobs, dungeons);
    let bird_castle = data.dungeons.get("bird_castle").unwrap().clone();
    let dungeon_name = bird_castle.name();

    println!("Welcome back, timeless!");
    println!("You enter the dungeon {dungeon_name}.");

    let mut player_mob = data.mobs.get("phoenix").unwrap().clone();
    let sleep_time = time::Duration::from_millis(1000);

    for mob in bird_castle {
        let mut mob = data.mobs.get(&mob).unwrap().clone();
        let drop = mob.drop();
        println!("You encountered {}.", mob.name().red());

        loop {
            let roll = player_mob.attack_roll();
            mob.add_health(-roll);
            thread::sleep(sleep_time.clone());
            println!("{} attacks {} for {roll} points of damage.", player_mob.name().green(), mob.name().red());

            if mob.is_dead() {
                println!("You defeated {}.", mob.name());
                break;
            }

            let roll = mob.attack_roll();
            player_mob.add_health(-roll);
            thread::sleep(sleep_time.clone());
            println!("{} attacks {} for {roll} points of damage", mob.name().red(), player_mob.name().green());

            if player_mob.is_dead() {
                println!("Your mob {} is defeated.", player_mob.name());
                break;
            }
        }

        if player_mob.is_dead() {
            println!("You had to run out of the dungeon.");
            break;
        }

        data.inventory.gold += drop.gold;
        if !drop.items.is_empty() {
            for item in &drop.items {
                let i = data.items.get(&item.1).unwrap();
                data.inventory.add(i, item.0 as i32);
            }
        }
        println!("{:?}", data.inventory);
    }

    if !player_mob.is_dead() {
        println!("You successfully beat the dungeon {dungeon_name}.");
    }
}