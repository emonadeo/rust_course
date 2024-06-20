fn print_window(ball_x: i32, ball_y: i32, width: i32, height: i32) {
    print!("{}[2J", 27 as char); // clear screen
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            if x == ball_x && y == ball_y {
                output.push('O');
            } else {
                output.push(' ');
            }
        }
        output.push('|');
        output.push('\n');
    }
    for _ in 0..width {
        output.push('-');
    }
    println!("{}", output);
}

fn main() {
    let width: i32 = 60;
    let height: i32 = 40;
    let mut ball_x: i32 = width / 2;
    let mut ball_y: i32 = height / 2;
    let mut ball_x_speed: i32 = 1;
    let mut ball_y_speed: i32 = 1;
    loop {
        print_window(ball_x, ball_y, width, height);
        // Ball bounce
        if ball_x == 0 || ball_x == width - 1 {
            ball_x_speed = -ball_x_speed;
        }
        if ball_y == 0 || ball_y == height - 1 {
            ball_y_speed = -ball_y_speed;
        }
        // Ball move
        ball_x += ball_x_speed;
        ball_y += ball_y_speed;
        std::thread::sleep(std::time::Duration::from_millis(1000 / 60)); // ~60 fps
    }
}
