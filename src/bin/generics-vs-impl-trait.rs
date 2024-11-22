struct Dog {
    name: String,
}

trait Pet {
    fn talk(&self);
}

impl Pet for Dog {
    fn talk(&self) {
        let woof = format!("Woof, my name is {}!", self.name);
        println!("{}", woof);
    }
}

fn generics_demo<T: Pet>(pet: &T) {
    pet.talk();
}


fn impl_demo(pet: impl Pet) {
    pet.talk();
}

// Note:
// For performance reasons, we should prefer generics over dynamic call (`e.g., impl some_trait)`, since Rust compiler will resolve
// generics to concrete struct calls at compile time, hence achieving static method calls (& not dynamic method call via vtable at runtime)

fn main() {
    let dog = Dog {
        name: String::from("Fido"),
    };

    generics_demo(&dog);
    impl_demo(dog);
}