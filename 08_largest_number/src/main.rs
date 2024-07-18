#[derive(Clone, Copy, Debug)]
enum Number {
    Int(i32),
    Float(f32),
}

fn largest(list: Vec<Number>) -> Number {
    // ...
}

fn main() {
    let number_list: Vec<Number> = vec![
        Number::Int(34),
        Number::Float(2.5),
        Number::Float(123.45),
        Number::Int(50),
    ];
    let result = largest(number_list);
    println!("The largest number is {:?}", result);
}
