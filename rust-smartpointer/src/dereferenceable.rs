use std::ops::Deref;

struct MyDereferenceable {
    value: String,
}

impl MyDereferenceable {
    fn new() -> Self {
        Self {
            value: String::from("dereferenced"),
        }
    }
}

impl Deref for MyDereferenceable {
    type Target = String;

    fn deref(&self) -> &String {
        &self.value
    }
}

fn main() {
    let my_deref = MyDereferenceable::new();
    println!("{:?}", *my_deref);
}
