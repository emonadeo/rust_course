# Ownership and Borrowing

> [!IMPORTANT]
> This script is very derivative of the [Ownership Chapter](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html#understanding-ownership) in the Rust Book.
> It is encouraged to also read it if you are looking for more details

## Recap

1. `for` loop
2. Limiting the number of guesses in the guessing game
3. Functions, Parameters, Return Values
4. Writing a function `find_in_vector` that checks if a given Vector contains a given Element

## Ownership and Borrowing

Consider the following code.

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

If you have worked with other programming languages you would expect this to work,
but instead to get an error: „borrow of moved value: `my_name`“.

Let's run `cargo build` to get a better picture of the error.

```console
error[E0382]: borrow of moved value: `my_name`
 --> src/main.rs:8:28
  |
6 |     let my_name = "Emanuel".to_string();
  |         ------- move occurs because `my_name` has type `String`, which does not implement the `Copy` trait
7 |     greet(my_name);
  |           ------- value moved here
8 |     println!("Goodbye {}", my_name);
  |                            ^^^^^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `greet` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:16
  |
1 | fn greet(name: String) {
  |    -----       ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
```

> [!TIP]
> Errors highlighted in your Text Editor quickly become difficult to look at.
> It is recommended to run `cargo build` in the terminal anyways,
> as the Rust Compiler provides a detailed overview and usually very useful hints.

The error boils down to three keywords:

-   Borrowed
-   Moved
-   Ownership

### Ownership

At some point, every compiler has to address the elephant in the room: **Memory Management**

Every variable and every data structure you create and use in your program obviously needs to be stored somewhere.
In most cases these are stored in memory.

But memory is finite, and all programs that run on your computer need to read and write to it.

While it is straight forward enough to **allocate** new memory, we also want to to **free** the memory,
as soon as we don't need it anymore, so we don't needlessy take up valuable memory space.

A few different techniques exist:

-   **Manual Management**, like C or Assembly
-   **Garbage Collector**, like in Java or Python
-   **Automatic Reference Couting**, like Swift
-   **Ownership-Model**, like Rust or... well so far only Rust has really pulled it off

Ownership is Rust’s most unique feature and has deep implications for the rest of the language.

#### Ownership Rules

The Ownership-Model consists of a set of rules that must be true at any point in the program:

1. Each value in Rust has an **owner**
2. There can be only one owner at a time
3. When the owner goes out of scope, the value will be dropped (i.e. the memory will be freed).

If your code breaks any of those rules at any point, it's not a valid Rust program and will be rejected by the compiler.

#### Scopes

As a first example of ownership, we’ll look at the scope of some variables.
A scope is the range within a program for which an item is valid. Take the following variable:

```rust
let s = "hello";
```

The variable is valid from the point at which it’s declared until the end of the current **scope**.

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

> [!TIP]
> A scope is denoted by curly braces (`{}`).  
> Functions also have their own scope (Parameters treated like Variables).

### To be continued in [Slides](https://github.com/pfhaupt/progkurs/blob/master/rust-beginner/05%20-%20Ownership%20and%20Borrow%20Checker/05%20-%20RUSTikales%20Rust%20for%20beginners.pptx) by [Philippe Felix Haupt](https://github.com/pfhaupt).

> [!NOTE]
> The slides are currently in `.pptx`.
> They will be available as `.pdf` next week.

### Borrowing using Immutable References

See [Slides](https://github.com/pfhaupt/progkurs/blob/master/rust-beginner/05%20-%20Ownership%20and%20Borrow%20Checker/05%20-%20RUSTikales%20Rust%20for%20beginners.pptx) by [Philippe Felix Haupt](https://github.com/pfhaupt).

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

Let's take a look at a similar case to the one introduced in the very beginning, but instead of using a `String` we use a number (`u16`).

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

This works because values of primitive datatypes (like `i8` &ndash; `i128`, `u8` &ndash; `u128`, `isize`, `usize`, `f32`, `f64`, `bool`, `char`) are always copied and never moved.

See: https://doc.rust-lang.org/stable/std/marker/trait.Copy.html#implementors

### Exercise

`->` `05_find_in_vector_advanced/`

Take the last week's exercise `find_in_vector`.

As seen in the code below, we now want to find two different numbers in the same vector.
However when we try to run this program, we get an error.

How can we fix the program without creating multiple vectors?

```rust
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
    println!("{}", a);

    let b = find_number(haystack, 5); // ERROR: use of moved value: `haystack`
    println!("{}", b);
}
```

### Borrowing using Mutable References

> [!Important]
> You can **either** have _1 mutable reference_ to a value **or** _any number of immutable references_.

See [Slides](https://github.com/pfhaupt/progkurs/blob/master/rust-beginner/05%20-%20Ownership%20and%20Borrow%20Checker/05%20-%20RUSTikales%20Rust%20for%20beginners.pptx) by [Philippe Felix Haupt](https://github.com/pfhaupt).
