trait CharacterClass {
    type Weapon: Weapon;

    fn create_weapon() -> Self::Weapon;
}

trait Weapon {
    fn attack(&self);
}

struct Sword;
impl Weapon for Sword {
    fn attack(&self) {
        println!("Attacking with sword...");
    }
}

struct Warrior;
impl CharacterClass for Warrior {
    type Weapon = Sword;
    fn create_weapon() -> Self::Weapon {
        Sword
    }
}

fn main() {
    Warrior::create_weapon().attack();
}