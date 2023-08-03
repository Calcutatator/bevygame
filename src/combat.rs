use crate::{Person, Monster};
//create a function to battle a person against a monster
//this function will take in a person and a monster
//will create a battle stat based on the persons stat type and weapon strength 
//will create a monster_battle_stat based on the monsters stat type and weapon strength
//it will return a bool
//the bool will be true if the person wins, false if the monster wins

pub fn combat(person: Person, monster: Monster) -> bool {
    //create a battle stat based on the persons stat type and weapon strength 
    let battle_stat: i32 = person.stat * person.weapon.strength;
    //create a monster_battle_stat based on the monsters stat type and weapon strength
    let monster_battle_stat: i32 = monster.stat * monster.weapon.strength;
    //if the battle stat is greater than the monster_battle_stat
    if battle_stat > monster_battle_stat {
        //return true
        return true;
    }
    //else
    else {
        //return false
        return false;
    }
}
//create a implementation of the battle function
// A random number generator will pick a monster from the monster list
// The monster will be passed into the battle function

pub fn battle(person: Person) {
    //create a list of monsters
    let monster_list: [Monster; 3] = [
        Monster {
            monster_name: "Goblin".to_string(),
            monster_stat: 3,
            monster_hp: 1,
            monster_weapon: MonsterWeapon {
                name: "Dagger".to_string(),
                strength: 1,
            },
        },
        Monster {
            monster_name: "Orc".to_string(),
            monster_stat: 5,
            monster_hp: 2,
            monster_weapon: MonsterWeapon {
                name: "Sword".to_string(),
                strength: 2,
            },
        },
        Monster {
            monster_name: "Dragon".to_string(),
            monster_stat: 8,
            monster_hp: 3,
            monster_weapon: MonsterWeapon {
                name: "Fire Breath".to_string(),
                strength: 3,
            },
        },
    ];
    //create a random number generator
    let mut rng = thread_rng();
    //create a random number between 0 and 2
    let random_number: u8 = rng.gen_range(0..2);
    //create a monster from the monster list based on the random number
    let monster: Monster = monster_list[random_number as usize];
    //call the battle function
    let battle_result: bool = combat(person, monster);
}

//create a function to print the battle results
//this function will take in a bool
//if the bool is true
//print "You won!"
//else
//print "You lost!"
pub fn print_battle_results(battle_result: bool) {
    //if the bool is true
    if battle_result == true {
        //print "You won!"
        println!("You won!");
    }
    //else
    else {
        //print "You lost!"
        println!("You lost!");
    }
}

