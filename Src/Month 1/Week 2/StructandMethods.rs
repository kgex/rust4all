// Define a struct
struct Person {
    name: String,
    age: u32,
}

// Implement methods for the struct
impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn introduce(&self) {
        println!("Hi, my name is {} and I'm {} years old.", self.name, self.age);
    }
}

fn main() {
    let person = Person::new("Alice", 25);
    person.introduce();
}
