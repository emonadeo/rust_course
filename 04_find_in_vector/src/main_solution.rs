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
    let a = find_number(haystack, 3);
    println!("{}", a); // true

    let haystack2 = vec![2, 4, 7, 12];
    let b = find_number(haystack2, 5);
    println!("{}", b); // false
}
