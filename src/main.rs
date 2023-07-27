use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new().add_startup_system(setup)
    .add_startup_system(create_window)
    .add_startup_system(print_stats)
    .add_startup_system(print_weapons)
    .add_plugins(DefaultPlugins)
    .run()
}

fn create_window(mut commands: Commands) {
    commands.spawn(Window {
        title: "Battle of DOOOOOM".to_string(),
        resolution: (WIDTH, HEIGHT).into(),
        ..default()
    });
}

pub fn setup(mut commands: Commands)    {
    commands.spawn((
        Person {
        name: "Alex".to_string(),
        stat: 5,
        weapon: "Broadsword".to_string(),
    },  
        Health {
        hp: 2
    },));
    
    commands.spawn((
        Person {
        name: "Bert".to_string(),
        stat: 8,
        weapon: "Vape".to_string(),
    }, 
        Health {
        hp: 3
    },));

    commands.spawn((
        Person {
        name: "Cara".to_string(),
        stat: 6,
        weapon: "Nunchucks".to_string(),
    }, 
        Health{ 
        hp: 5
    },));
}


pub fn print_stats(query: Query<(&Person, &Health)>) {
    for (person, health) in query.iter() {
        println!("Name: {}  Stat: {}    Health: {}  Weapon: {}", person.name, person.stat, health.hp, person.weapon);
    }
}

pub fn print_weapons(query_weapons: Query<&Person>) {
    let mut weapons: Vec<String> = Vec::new();

    for person in query_weapons.iter() {
        weapons.push(person.weapon.clone());
    }
    println!("Weapon options:");
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