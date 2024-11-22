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

struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

// Function takes a trait object reference
fn main() {
    let circle = Circle { radius: 2.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };
    print_area(&circle);
    print_area(&rectangle);
}