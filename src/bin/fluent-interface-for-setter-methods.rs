#[derive(Debug)]
struct Ticket1 {
    title: String,
    description: String,
}

// Approach 1: Pass `mut self` each time & return `Self`
impl Ticket1 {
    pub fn set_title(mut self, new_title: String) -> Self {
        self.title = new_title;
        self
    }

    pub fn set_description(mut self, new_description: String) -> Self {
        self.description = new_description;
        self
    }
}

#[derive(Debug)]
struct Ticket2 {
    title: String,
    description: String,
}

// Approach 2: Pass `&mut self` each time & return `&mut Self`
impl Ticket2 {
    pub fn set_title(&mut self, new_title: String) -> &mut Self {
        self.title = new_title;
        self
    }

    pub fn set_description(&mut self, new_description: String) -> &mut Self {
        self.description = new_description;
        self
    }
}

// Fluent interface for setter methods (using 2 approaches)
fn main() {
    let mut ticket1 = Ticket1 { title: "".to_string(), description: "".to_string() };
    ticket1 = ticket1
        .set_title("Title1".into())
        .set_description("Description1".into());
    println!("{:#?}", ticket1);

    let mut ticket2 = Ticket2 { title: "".to_string(), description: "".to_string() };
    ticket2
        .set_title("Title2".into())
        .set_description("Description2".into());
    println!("{:#?}", ticket2);
}