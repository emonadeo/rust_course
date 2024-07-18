#[derive(Clone, Copy, Debug)]
enum Number {
    Float(f32),
    Int(i32),
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Number::Int(item_int), Number::Int(largest_int)) => {
                return item_int.partial_cmp(largest_int);
            }
            (Number::Int(item_int), Number::Float(largest_float)) => {
                return (&(*item_int as f32)).partial_cmp(largest_float);
            }
            (Number::Float(item_float), Number::Int(largest_int)) => {
                return item_float.partial_cmp(&(*largest_int as f32));
            }
            (Number::Float(item_float), Number::Float(largest_float)) => {
                return item_float.partial_cmp(largest_float);
            }
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Int(item_int), Number::Int(largest_int)) => {
                return item_int.eq(largest_int);
            }
            (Number::Int(item_int), Number::Float(largest_float)) => {
                return (&(*item_int as f32)).eq(largest_float);
            }
            (Number::Float(item_float), Number::Int(largest_int)) => {
                return item_float.eq(&(*largest_int as f32));
            }
            (Number::Float(item_float), Number::Float(largest_float)) => {
                return item_float.eq(largest_float);
            }
        }
    }
}

fn largest(list: Vec<Number>) -> Number {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

pub fn main() {
    let number_list: Vec<Number> = vec![
        Number::Int(34),
        Number::Float(2.5),
        Number::Float(123.45),
        Number::Int(50),
    ];
    let result = largest(number_list);
    println!("The largest number is {:?}", result);
}
