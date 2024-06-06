//                       ↙ The data type of the haystack also needs to be changed to a reference.
fn find_number(haystack: &Vec<i32>, needle: i32) -> bool {
    for number in haystack {
        // EITHER...
        // ↙ Dereference the number
        if *number == needle {
            return true;
        }
        // OR...
        //           ↙ Borrow the needle
        if number == &needle {
            return true;
        }
    }
    return false;
}

fn main() {
    let haystack = vec![2, 4, 7, 12];
    //                    Instead of passing the haystack as a value,
    //                  ↙ we pass it as an immutable reference.
    let a = find_number(&haystack, 2);
    println!("{}", a); // true

    //                    We can have as many immutable references
    //                  ↙ to the haystack as we want.
    let b = find_number(&haystack, 5); // No error :)
    println!("{}", b); // false
}
