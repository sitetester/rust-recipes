
// Add custom method to built-in types (extension trait)
fn main() {
    trait BoolTrait {
        fn is_true(&self) -> bool;
        fn is_false(&self) -> bool;
    }

    impl BoolTrait for bool {
        fn is_true(&self) -> bool {
            *self == true
        }
        fn is_false(&self) -> bool {
            *self == false
        }
    }

    let b = true;
    println!("{:?} {:?}", b.is_true(), b.is_false());
}