use bevy::prelude::*;

fn main() {
    App::new().add_startup_system(setup)
    .add_system(print_stats)
    .add_system(print_weapons)
    .run()
}

pub fn setup(mut commands: Commands)    {
    commands.spawn((
        Person {
        name: "Alex".to_string(),
        stat: 5,
        weapon: "Broadsword".to_string(),
    },  
        Health {
        hp: 10
    },));
    
    commands.spawn((
        Person {
        name: "Berty".to_string(),
        stat: 8,
        weapon: "Vape".to_string(),
    }, 
        Health {
        hp: 6
    },));

    commands.spawn((
        Person {
        name: "Cara".to_string(),
        stat: 3,
        weapon: "Nunchucks".to_string(),
    }, 
        Health{ 
        hp: 5
    },));
}


pub fn print_stats(query: Query<(&Person, &Health)>) {
    for (person, health) in query.iter() {
        println!("Name: {}", person.name);
        println!("Stat: {}", person.stat);
        println!("Weapon: {}", person.weapon);
        println!("Health: {}", health.hp);
    }
}

pub fn print_weapons(query_weapons: Query<&Person>) {
    let mut weapons: Vec<String> = Vec::new();

    for person in query_weapons.iter() {
        weapons.push(person.weapon.clone());
    }
    println!("{:?}", weapons);
}


#[derive(Component)] 
pub struct Person {
    name: String,
    stat: i32,
    weapon: String,
}
#[derive(Component)] 
pub struct Health {
    hp: u8,
}
