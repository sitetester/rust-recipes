struct A;

trait Foo {
    fn foo(&self);
}

trait Bar {
    fn bar(&self);
}

impl Foo for A {
    fn foo(&self) {
        println!("A foo")
    }
}

impl Bar for A {
    fn bar(&self) {
        println!("A bar")
    }
}

trait FooBar: Foo + Bar{}
impl FooBar for A {}

// Type param bound by `Foo + Bar`
fn foo_and_bar<T: Foo + Bar>(f: &T) {
    f.foo();
    f.bar();
}

// We can also use a sinlge trait instead which is composed of multiple independent traits
// Type param bound by `FooBar`
fn foobar<T: FooBar>(f: T) {
    f.foo();
    f.bar();
}

fn main() {
    let a = A;
    foo_and_bar(&a);

    foobar(a);
}