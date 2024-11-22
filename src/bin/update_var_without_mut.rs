use core::cell::Cell;
use std::cell::RefCell;
use std::sync::{Mutex, RwLock};

fn shadow_demo() -> i32 {
    let age = 20;

    // it's not really mutating, it just blocks/overrides the previous var (both vars still exist in memory)
    // will not work if have different scopes
    let age = 30;
    age
}

fn cell_primitive_demo() -> i32 {
    let age = Cell::new(20);
    age.set(30);
    age.get()
}

fn nested_cell_demo() -> i32 {
    struct User {
        age: Cell<i32>,
    }

    let user = User { age: Cell::new(20) };

    user.age.set(30);
    user.age.get()
}
// it stands for mutual exclusion, and it's the simplest kind of lock:
// it allows only one thread to access the data, no matter if it's for reading or writing
fn mutex_demo() -> i32 {

    let v = Mutex::new(20);

    // guard object that dereferences to the data (via `Deref` implementation), allowing us to modify it
    // the lock will be released when the guard goes out of scope (via `Drop` implementation)

    // Mutex<T> is a smart pointer. More accurately, the call to lock returns a smart pointer
    // called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap
    let mut guard = v.lock().unwrap(); // can also use `try_lock` here
    *guard = 30;

    *guard
}

fn rw_lock_demo() -> i32 {
    let v = RwLock::new(20);

    let mut guard = v.write().unwrap();
    *guard = 30;

    *guard
}

fn ref_cell_demo() -> i32 {
    let x = RefCell::new(42);
    {
        let mut y = x.borrow_mut();
        *y = 43;
    }

    let x = *x.borrow();
    x
}

// the usage shows how you can borrow and mutate the passed field
// even though student itself is immutable after creation
#[derive(Debug)]
struct Student {
    id: u32,
    passed: RefCell<bool>,
}
fn interior_mutability_demo() -> bool {
    let student = Student {
        id: 123,
        passed: RefCell::new(false),
    };

    // can also do like this
    // *student.passed.borrow_mut() = true;s
    // let passed = *student.passed.borrow();

    student.passed.replace(true);
    student.passed.take()
}

// Update a Rust variable without declaring it with `mut`
fn main() {
    println!("shadow_demo: {}", shadow_demo());
    println!("cell_primitive_demo: {}", cell_primitive_demo());
    println!("nested_cell_demo: {:?}", nested_cell_demo());
    println!("mutex_demo: {:?}", mutex_demo());
    println!("rw_lock_demo: {:?}", rw_lock_demo());
    println!("ref_cell_demo: {:?}", ref_cell_demo());
    println!("interior_mutability_demo: {:?}", interior_mutability_demo());
}