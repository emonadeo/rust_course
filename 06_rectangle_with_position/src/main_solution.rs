struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    position: Point,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        return Rectangle {
            position: Point { x: 0, y: 0 },
            width,
            height,
        };
    }

    fn print(&self) -> () {
        for _ in 0..self.position.y {
            let total_width = self.position.x + self.width;
            for _ in 0..total_width {
                print!(".");
            }
            print!("\n");
        }
        for _ in 0..self.height {
            for _ in 0..self.position.x {
                print!(".");
            }
            for _ in 0..self.width {
                print!("#");
            }
            print!("\n");
        }
    }
}

fn main() {
    let my_rectangle = Rectangle {
        position: Point { x: 5, y: 2 },
        width: 10,
        height: 3,
    };
    my_rectangle.print();
}
