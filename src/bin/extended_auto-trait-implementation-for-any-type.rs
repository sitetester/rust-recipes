trait Draw {
    fn draw(&self);
}

trait Print {
    fn print(&self);
}

#[derive(Debug)]
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
fn main() {
    let circle = Circle { radius: 3.5 };
    circle.draw();
}