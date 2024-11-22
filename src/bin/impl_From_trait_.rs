use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Department {
    employee: Vec<Employee>,
}

#[derive(Debug)]
struct Employee {
    name: String,
}

impl From<Vec<Employee>> for Department {
    fn from(employee_vec: Vec<Employee>) -> Self {
        Department { employee: employee_vec }
    }
}

impl Display for Department {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = format!("Department has {} employees\n", self.employee.len());
        for emp in &self.employee {
            str.push_str(&emp.name);
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

// `pub trait From<T>` implementation demo
fn main() {
    let dept = Department::from(vec![
        Employee {
            name: "Alex".to_string(),
        },
        Employee {
            name: "John".to_string(),
        },
    ]);

    println!("{dept}");
}