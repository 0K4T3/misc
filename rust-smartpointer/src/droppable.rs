use std::ops::Drop;

struct MyDroppable {}

impl Drop for MyDroppable {
    fn drop(&mut self) {
        println!("dropped");
    }
}

fn main() {
    println!("before drop");
    {
        let droppable = MyDroppable {};
    }
    println!("after drop");
}
