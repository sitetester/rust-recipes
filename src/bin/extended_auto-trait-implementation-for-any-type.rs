trait Draw {
    fn draw(&self);
}

#[allow(dead_code)]
trait Print {
    fn print(&self);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Circle {
    radius: f32,
}

// Implementation type must implement `Print + std::fmt::Debug` in order to implement `Draw` trait
impl<T> Draw for T
where
    T: Print + std::fmt::Debug,
{
    fn draw(&self) {
        println!("Drawing: {:?}", self);
    }
}

impl Print for Circle {
    fn print(&self) {
        println!("Printing: {}", self.radius);
    }
}

// It's an extended version of previous demo
// Auto implementation of some trait T for some type (e.g. struct) when it already implements another trait
fn main() {
    let circle = Circle { radius: 3.5 };
    circle.draw();
}