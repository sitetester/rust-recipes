trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Dynamically Sized Types (DSTs)
// DSTs are types that have a size unknown at compile time.
// This is an example of a dynamically sized type because the concrete type of circle is determined at runtime.
fn main() {
    // Create a trait object circle referring to a Circle instance.
    let circle: &dyn Shape = &Circle { radius: 5.0 };

    // area method on the circle trait object.
    println!("Circle area: {}", circle.area());
}