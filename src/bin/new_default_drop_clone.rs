struct User {
    name: String,
}

impl User {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }

    fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

// another way to initialize a struct
impl Default for User {
    fn default() -> Self {
        User { name: "Alex".to_string() }
    }
}

impl Drop for User {
    fn drop(&mut self) {
        println!("destructure called for {}!", self.name);
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        println!("Cloning User {}...", self.name);
        Self {
            name: self.name.clone()
        }
    }
}

// this function will simply take ownership & hence it's destructure should be called eventually
fn consume_user(_user: User) {
    // do nothing
}

fn main() {
    let u1 = User::new("u1");
    consume_user(u1);
    println!("still in main()");

    let mut u2 = User::new("u2");
    {
        // User struct is a dynamic type (allocated on heap) & assignment will move ownership,
        // hence it's destructure will also be invoked in this case
        let u3 = &mut u2;
        u3.name("u3");
    }
    println!("still in main()");

    let u4 = User::new("u4");
    let mut u5 = u4.clone();
    u5.name("u5");

    let u6 = User::default();
    assert_eq!(u6.name, "Alex");

    #[derive(Default, PartialEq, Debug)]
    struct User1 {
        fname: String,
        lname: String,
    }

    // usage of `Default::default()`
    let user1_1 = User1 {
        fname: "Alex".to_string(),
        ..Default::default()
    };
    assert_eq!(user1_1, User1{fname: "Alex".to_string(), lname: "".to_string()});

    // usage of `User1::default()`
    let user1_2 = User1 {
        fname: "Alex".to_string(),
        ..User1::default()
    };
    assert_eq!(user1_2, User1{fname: "Alex".to_string(), lname: "".to_string()});


}