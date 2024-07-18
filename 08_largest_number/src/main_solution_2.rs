#[derive(Clone, Copy, Debug)]
enum Number {
    Int(i32),
    Float(f32),
}

fn largest(list: Vec<Number>) -> Number {
    let mut largest = list[0];
    for item in list {
        // Instead of nested match statements, we can use a single match statement
        // on the tuple `(item, largest)` to compare the two numbers.
        match (item, largest) {
            (Number::Int(item_int), Number::Int(largest_int)) => {
                // Comparing Int to Int
                if item_int > largest_int {
                    largest = item;
                }
            }
            (Number::Int(item_int), Number::Float(largest_float)) => {
                // Comparing Int to Float
                if item_int as f32 > largest_float {
                    largest = item;
                }
            }
            (Number::Float(item_float), Number::Int(largest_int)) => {
                // Comparing Float to Int
                if item_float > largest_int as f32 {
                    largest = item;
                }
            }
            (Number::Float(item_float), Number::Float(largest_float)) => {
                // Comparing Float to Float
                if item_float > largest_float {
                    largest = item;
                }
            }
        }
    }
    return largest;
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
