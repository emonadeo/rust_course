struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        return Rectangle { width, height };
    }

    fn print(&self) -> () {
        for _ in 0..self.height {
            for _ in 0..self.width {
                print!("#");
            }
            print!("\n");
        }
    }
}

fn main() {
    let rect = Rectangle::new(10, 3);
    rect.print();
}
