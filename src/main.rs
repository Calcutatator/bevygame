use bevy::prelude::*;
    
fn main() {
    App::new().add_startup_system(setup)
    .add_startup_system(monster_setup)
    .add_startup_system(decision)
    .add_startup_system(combat)
    .run()
}
//components
#[derive(Component)] 
pub struct Person {
    name: String,
    stat: i32,
    weapon: String,
    hp: u8,
}
#[derive(Component)]
pub struct Monster {
    monster_name: String,
    monster_stat: i32,
    monster_hp: u8,
    monster_weapon: String,
}
#[derive(Component)] 
pub enum Weapon {
    Broadsword = 5,
    Vape = 8,
    Nunchucks = 6,
    Dagger = 2,
    Sword = 4,
    Club = 3,
}

//person setup
pub fn setup(mut commands: Commands)    {
    commands.spawn((
        Person {
        name: "Alex".to_string(),
        stat: 5,
        weapon: "Broadsword".to_string(),
        hp: 2,
    },));
    
    commands.spawn((
        Person {
        name: "Bert".to_string(),
        stat: 8,
        weapon: "Vape".to_string(),
        hp: 1,
    },));

    commands.spawn((
        Person {
        name: "Cara".to_string(),
        stat: 6,
        weapon: "Nunchucks".to_string(),
        hp: 3,
    },));
}
//monster setup
pub fn monster_setup(mut commands: Commands)    {
    commands.spawn((
        Monster {
        monster_name: "Goblin".to_string(),
        monster_hp: 2,
        monster_weapon: "Dagger".to_string(),
        monster_stat: 2,
    },));
    commands.spawn((
        Monster {
        monster_name: "Orc".to_string(),
        monster_hp: 3,
        monster_weapon: "Sword".to_string(),
        monster_stat: 4,
    },));
    commands.spawn((
        Monster {
        monster_name: "Troll".to_string(),
        monster_hp: 5,
        monster_weapon: "Club".to_string(),
        monster_stat: 3,
    },));
}

//create an input function from the user
//this will result in an option of a person and a monster
pub fn decision() -> Option<(Person, Monster)> {
    //create a new string
    let mut input = String::new();
    //get the user input
    std::io::stdin().read_line(&mut input).unwrap();
    //split the input into a vector
    let input: Vec<&str> = input.split_whitespace().collect();
    //if the input is not empty
    if !input.is_empty() {
        //return the input
        return Some((Person {
            name: input[0].to_string(),
            stat: input[1].parse::<i32>().unwrap(),
            weapon: input[2].to_string(),
            hp: input[3].parse::<u8>().unwrap(),
        }, Monster {
            monster_name: input[4].to_string(),
            monster_stat: input[5].parse::<i32>().unwrap(),
            monster_hp: input[6].parse::<u8>().unwrap(),
            monster_weapon: input[7].to_string(),
        }));
    }
    //else
    else {
        //return none
        return None;
    }
}

pub fn combat(person: Person, monster: Monster) {
    //create a battle stat based on the persons stat type and weapon type value 
    let battle_stat: i32 = person.stat * match person.weapon.as_str() {
        "Broadsword" => Weapon::Broadsword as i32,
        "Vape" => Weapon::Vape as i32,
        "Nunchucks" => Weapon::Nunchucks as i32,
        _ => 0,
    };
    //create a monster_battle_stat based on the monsters stat type and weapon type value
    let monster_battle_stat: i32 = monster.monster_stat * match monster.monster_weapon.as_str() {
        "Dagger" => Weapon::Dagger as i32,
        "Sword" => Weapon::Sword as i32,
        "Club" => Weapon::Club as i32,
        _ => 0,
    };
    if battle_stat > monster_battle_stat {
        return print_battle_results(true);
    }
    else {
        return print_battle_results(false);
    }
}

pub fn print_battle_results(battle_result: bool) {
    if battle_result == true {
        println!("You won!");
    }
    else {
        println!("You lost!");
    }
}



// A function to print all of the person stats
pub fn print_stats(query: Query<&Person>) {
    for person in query.iter() {
        println!("Name: {}  Stat: {}    Health: {}  Weapon: {}", person.name, person.stat, person.hp, person.weapon);
    }
}

// A function to print all of the monster stats
pub fn print_monster_stats(query: Query<&Monster>) {
    for monster in query.iter() {
        println!("Name: {}  Health: {}  Weapon: {} Stat: {}", monster.monster_name, monster.monster_hp, monster.monster_weapon, monster.monster_stat);
    }
}

// Print weapon types function
pub fn print_weapons(query_weapons: Query<&Person>) {
    let mut weapons: Vec<String> = Vec::new();

    for person in query_weapons.iter() {
        weapons.push(person.weapon.clone());
    }
    println!("Weapon options:");
    println!("{:?}", weapons);
}



