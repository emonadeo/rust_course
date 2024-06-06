fn find_number(haystack: Vec<i32>, needle: i32) -> bool {
    for number in haystack {
        if number == needle {
            return true;
        }
    }
    return false;
}

fn main() {
    let haystack = vec![2, 4, 7, 12];
    let a = find_number(haystack, 2);
    println!("{}", a); // true

    let b = find_number(haystack, 5); // Error: use of moved value: `haystack` :(
    println!("{}", b); // false
}
