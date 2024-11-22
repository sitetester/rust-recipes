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

// Type-level Programming (when a trait has a (placeholder) type which is itself a trait)
fn main() {
    Warrior::create_weapon().attack();
}