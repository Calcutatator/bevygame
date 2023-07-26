use bevy::prelude::*;

fn main() {
    App::new().add_startup_system(setup)
    .add_system(print_stats)
    .run()
}

pub fn setup(mut commands: Commands)    {
    commands.spawn((Person {
        name: "Alex".to_string(),
        stat: 5,
        weapon: "Broadsword".to_string(),
    },  Health {hp: 10},));
    
    commands.spawn((Person {
        name: "Berty".to_string(),
        stat: 8,
        weapon: "Vape".to_string(),
    }, Health {hp: 6},));

    commands.spawn((Person {
        name: "Cara".to_string(),
        stat: 3,
        weapon: "Nunchucks".to_string(),
    }, Health{ hp: 5},));
}


pub fn print_stats(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
        println!("Stat: {}", person.stat);
        println!("Weapon: {}", person.weapon);
    }
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
