## Recap

### Statements vs. Expressions

-   Statements are instructions that perform some action and do not return a value.
-   Expressions represent or evaluate to a single resultant value.

See: [Rust Book](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions)

## Ownership

```rust
fn greet(name: String) {
    println!("Hello {}.", name);
}

fn main() {
    let my_name = "Emanuel".to_string();
    greet(my_name);
    println!("Goodbye {}", my_name); // Error: borrow of moved value: `my_name`
}
```

### Borrowing

```rust
fn greet(name: &String) {
    println!("Hello {}.", name);
}

fn main() {
    let my_name = "Emanuel".to_string();
    greet(&my_name);
    println!("Goodbye {}", my_name);
}
```

### The `Copy` Trait

Let's take a look at a similar case, but instead of using a `String` we use a number (`u16`).

```rust
fn greet_age(age: u16) {
    println!("You are {} years old.", age);
}

fn main() {
    let my_age = 23
    greet_age(my_age);
    println!("Next year, you will be {} years old", my_age + 1);
}
```

You'll notice that the code compiles and runs successfully, without the need to borrow.

This works because values of primitive datatypes (like `i8` &ndash; `i128`, `u8` &ndash; `u128`, `isize`, `usize`, `f32`, `f64`, `bool`, `char`) are always copied and never borrowed.

See: https://doc.rust-lang.org/stable/std/marker/trait.Copy.html#implementors
