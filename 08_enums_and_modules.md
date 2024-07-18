# Enums & Modules

## Modules

[^1]

**Modules** let us organize code within a crate for readability and easy reuse.

We define a module with the `mod` keyword followed by the name of the module.

For example a module called `math`, containing a `square` function and `Rectangle` struct, may look like this:

```rust
mod math {
    fn square(n: i32) -> i32 {
        return n * n;
    }

    struct Rectangle {
        width: i32,
        height: i32,
    }
}
```

A member (functions, structs, constants, etc.) of a module can be referred using double colons (`::`).

```rust
mod math {
    fn square(n: i32) -> i32 {
        return n * n;
    }

    struct Rectangle {
        width: i32,
        height: i32,
    }
}

fn main() {
    let a = 2;
    let a_squared = math::square(2); // Compiler Error!
    println!("{}", a_squared);
}
```

However, this errors when you try to compile it.

Everything inside a module is **private**, meaning it cannot be accessed from outside.
To declare something as **public**, you have to prepend a member with the `pub` keyword.

```rust
mod math {
    pub fn square(n: i32) -> i32 {
        return n * n;
    }

    pub struct Rectangle {
        width: i32,
        height: i32,
    }
}

fn main() {
    let a = 2;
    let a_squared = math::square(2); // Works!
    println!("{}", a_squared);
}
```

### Splitting code into multiple files

As the code rapidly grows in size, you may want to split it into different `.rs` files.

This is the prime use-case for modules.

When compiling a crate, the compiler first looks in the crate root file
(usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate) for code to compile.

In the crate root file, you can declare new modules; say, you declare a “math” module with `mod math;`. The compiler will look for the module’s code in these places:

1. Inline, within curly brackets that replace the semicolon following `mod math`
2. In the file `src/math.rs`
3. In the file `src/math/mod.rs`

Applying this pattern to our `math` module, we can migrate the `square` function and `Rectangle` struct into a separate file like this:

**`src/main.rs`**

```rust
mod math;

fn main() {
    let a = 2;
    let a_squared = math::square(2); // Works!
    println!("{}", a_squared);
}
```

**`src/math.rs`** or **`src/math/mod.rs`**

```rust
pub fn square(n: i32) -> i32 {
    return n * n;
}

pub struct Rectangle {
    width: i32,
    height: i32,
}
```

### Paths to code in modules

Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code.

For example, an `Rectangle` struct in the `math` module would be found at `crate::math::Rectangle`.
The `crate` keyword **always** refers to the root module, i.e. the `src/main.rs` or `src/lib.rs` file, no matter where you are.

Similarly the `super` keyword refers to the **parent** module:

```rust
mod my_module {

    struct Thing {
        foo: i32
    }

    mod nested_module {

        fn do_something() {
            let my_thing = super::Thing {
                foo: 20
            };
        }

    }
}

```

### The `use` keyword

Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.
In any scope that can refer to `crate::garden::vegetables::Asparagus`,
you can create a shortcut with use `crate::garden::vegetables::Asparagus`;
and from then on you only need to write `Asparagus` to make use of that type in the scope.

```rust
mod garden {
    mod vegetables {
        pub struct Asparagus {
            // ...
        }
    }
}

use crate::garden::vegetables::Asparagus;

fn main() {
    //                 ↓ No need to write `crate::garden::vegetables::Asparagus`
    let my_asparagus = Asparagus {
        // ...
    };
}
```

This not only applies to functions and structs of a module, but also modules themselves:

```rust
mod garden {
    mod vegetables {
        pub struct Asparagus {
            // ...
        }
    }
}

// Create a shortcut to the `vegetables` module (but not Asparagus!)
use crate::garden::vegetables;

fn main() {

    //                 ↓ Use the shortcut here
    let my_asparagus = vegetables::Asparagus {
        // ...
    };
}
```

## Enums

In order to understand enums, it's useful to recap structs.
As we have learned, a struct lets us group together related data.

For example a struct `Point` may contain an x-coordinate and a y-coordinate

```rust
struct Point {
    x: i32,
    y: i32,
}
```

Once declared, a struct is also a new data type:

```rust
let my_point: Point = ...
```

Similar to structs, an **enum** also let's us define a new data type.
Enums give you a way of saying a value is one of a possible set of values.

For example, we may want to say that `Rectangle` is one of a set of possible shapes that also includes `Circle` and `Triangle`:

```rust
enum Shape {
    Circle,
    Square,
    Rectangle,
}
```

In this case, `Shape` is the **data type** . `Circle`, `Square` and `Rectangle` are **values**.

Once an enum is declared, you can access the values by using the `Shape` enum in conjunction with a double colon (`::`):

```rust
let my_circle_shape: Shape = Shape::Circle;
```

Let's assume we want to write a function `print_area_formula(shape: Shape)` which takes a `shape` parameter and prints its area formula.

We could use `if` conditions to check the shape value:

```rust
#[derive(Debug, PartialEq)] // <- needed for `println!` and `==` operations
enum Shape {
    Circle,
    Square,
    Rectangle,
}

fn print_area_formula(shape: Shape) {
    if shape == Shape::Circle {
        println!("πr²");
    } else if shape == Shape::Square {
        println!("a²");
    } else if shape == Shape::Rectangle {
        println!("ab");
    } else {
        println!("unknown shape");
    }
}

fn main() {
    let circle: Shape = Shape::Circle;
    let square: Shape = Shape::Square;
    let rectangle: Shape = Shape::Rectangle;

    print_area_formula(circle);
    print_area_formula(square);
    print_area_formula(rectangle);
}
```

> [!IMPORTANT]
> Quiz: Under what circumstances is „unknown shape“ printed?

### Data in Enums

Using enums has even more advantages.
Thinking more about our `Shape` type, at the moment we don’t have a way to store _**data**_ in those shapes; we only know what _**kind**_ it is.

For this, Rust allows you to attach data to enums directly

```rust
enum Shape {
    Circle(u32), // radius
    Square(u32), // width and height (same size)
    Rectangle(u32, u32), // width, height
}
```

```rust
fn main() {
    let circle: Shape = Shape::Circle(3);
    let bigger_circle: Shape = Shape::Circle(7);
    let square: Shape = Shape::Square(5);
    let rectangle: Shape = Shape::Rectangle(3, 5);
}
```

Great! Now we can have different shapes of different sizes.

However, many things have suddenly become a lot more difficult.
In particular, the `print_area_formula` no longer works, since the shape we compare against now also expects concrete data.

### Generic Enums and `Option<T>`

Unlike most other programming languages, Rust does not have `null`.
Instead, the abscence of a value is modeled using the `Option<T>` enum.

`Option<T>` is defined as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Notice, how the `Option` has a generic type paramater `T`.
This works exactly the same way for enums, as it does for structs.

For example substituing for `T` you get:

```rust
enum Option<i32> {
    None,
    Some(i32),
}
```

As you can see an Option has two variants:

1. `None`, describing the abscence of a value. In most other programming languages this would be modeled using `null`.
2. `Some(T)`, which mean there **is** a value of type `T`.

### `Result<T, E>`

A `Result<T, E>` is similar to an Option but with different semantics.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

It also has two variants:

1. `Ok(T)`, which means everything worked as expected, and holds a value representing the successful outcome.
2. `Err(E)`, which means something went wrong, and holds a value representing an error.

A practical example of the `Result` enum is a function opening and reading a file.

```rust
fn read_file(path: &str) -> Result<String, IoError> {
    // ...
}

enum IoError {
    FileNotFound,
    MissingPermission(String),
    // ...
}
```

This function takes a path argument, and either returns the contents of the file as a `String`, or an `IoError`, signaling what went wrong.

## Pattern Matching & `match` expressions

In order to compare the _**kind**_ (also called **variant**) of the shape while ignoring the actual _**data**_, we can't simply use an `if` statement.
Instead, Rust has `match` expressions and **pattern matching**, which is not only much more powerful, but also easier to read, faster, and altogether more convenient.

```rust
fn print_area_formula(shape: Shape) {
    match shape {
        Shape::Circle(_) => println!("πr²"),
        Shape::Square(_) => println!("a²"),
        Shape::Rectangle(_, _) => println!("ab"),
    }
}
```

The underscores (`_`) mean, that we do not care about the concrete value.
As long as `shape` is a `Circle`, we print `πr²`, no matter the radius.

We _could_ also „pattern match“ the concrete value if we wanted to:

```rust
fn print_area_formula(shape: Shape) {
    match shape {
        Shape::Circle(_) => println!("πr²"),
        Shape::Square(1) => println!("1"),
        Shape::Square(_) => println!("a²"),
        Shape::Rectangle(6, 9) => println!("nice"),
        Shape::Rectangle(_, 3) => println!("3a"),
        Shape::Rectangle(5, _) => println!("5b"),
        Shape::Rectangle(_, _) => println!("ab"),
    }
}
```

> [!IMPORTANT]
> The order of the match „branches“ is important.
> If multiple branches match the same pattern, the higher branch takes precedence.

```rust
fn main() {
    print_area_formula(Shape::Square(2)); // a²
    print_area_formula(Shape::Square(1)); // 1
    print_area_formula(Shape::Rectangle(2, 4)); // ab
    print_area_formula(Shape::Rectangle(6, 9)); // nice
    print_area_formula(Shape::Rectangle(5, 1)); // 5b
    print_area_formula(Shape::Rectangle(2, 3)); // 3a
}
```

> [!IMPORTANT] > `match`-Expressions are **exhaustive**.
> This means, you must cover every possible pattern.
> If the Rust compiler detects a pattern without a branch, it will not compile.

Additionally, matched values can be bound to a local variable and used inside the branch:

```rust
fn print_area_formula(shape: Shape) {
    match shape {
        Shape::Circle(radius) => println!("π *{}²", radius),
        Shape::Square(1) => println!("1"),
        Shape::Square(size) => println!("{}²", size),
        Shape::Rectangle(6, 9) => println!("nice"),
        Shape::Rectangle(_, 3) => println!("3a"),
        Shape::Rectangle(5, height) => println!("5 * {}", height),
        Shape::Rectangle(_, _) => println!("ab"),
    }
}
```

```rust
fn main() {
    print_area_formula(Shape::Square(2)); // 2²
    print_area_formula(Shape::Square(1)); // 1
    print_area_formula(Shape::Rectangle(2, 4)); // ab
    print_area_formula(Shape::Rectangle(6, 9)); // nice
    print_area_formula(Shape::Rectangle(5, 1)); // 5 * 1
    print_area_formula(Shape::Rectangle(2, 3)); // 3a
}
```

### Mutating inside a Pattern

Using `match`, you can also modify the original enum variant.

> [!NOTE]
> Every branch updates the original shape (notice the `mut`)

```rust
fn double_shape_size(shape: &mut Shape) {
    match shape {
        Shape::Circle(radius) => *radius = *radius * 2,
        Shape::Square(size) => *size = *size * 2,
        Shape::Rectangle(width, height) => {
            *width = *width * 2;
            *height = *height * 2;
        }
    }
}
```

### Pattern Matching Tuples and Structs

> [!NOTE]
> Consult the [Rust Book](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#pattern-syntax) for a list of all possible Patterns

You can also apply pattern matching to **tuples**...

```rust

fn main() {
    let p: (i32, i32) = (3, 8);

    match p {
        (6, 9) => println!("nice"),
        (3, _) => println!("the first element is a 3!"),
        (_, _) => println!("just a random tuple"),
    }
}
```

and **structs**...

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p: Point = Point { x: 0, y: 7 };

    match p {
        Point { x: 6, y: 9} => println!("nice"),
        Point { x: _, y: _} => println!("just a random point"),
    }
}
```

### Multiple Patterns

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### Matching Ranges of Values with `..=`

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

### Nested Pattern Matching

Pattern Matching can also be nested.

To demonstrate, we are using the `Result<T, E>` enum, and the `read_file` example from earlier.

```rust
enum IoError {
    FileNotFound,
    MissingPermission(String),
    // ...
}

fn read_file(path: &str) -> Result<String, IoError> {
    // ...
}

fn main() {
    let password_result = read_file("C:/Users/MaxMustermann/password.txt");
    match password_result {
        Ok(password) => println!("I got your password. It is: {}", password),
        Err(IoError::FileNotFound) => println!("password.txt does not exist"),
        //                             ↓ Nested Pattern!
        Err(IoError::MissingPermission(permission)) => println!("password.txt exists, but I am missing the {} permission.", permission)
    }
}

```

## Revisiting the `largest` function

Consider this function from the last chapter, which finds the largest `i32` in a list of `i32`:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
```

We ran into the problem, where we wanted to use the same algorithm to find the largest `f32` in a list of `f32`, without having to copy-paste code.

One way to solve this is using a generic type and the `std::cmp::PartialOrd` trait:

```rust
fn largest<T>(list: &[T]) -> &T
where T: std::cmp::PartialOrd {
    // ...
}
```

```rust
fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result: i32 = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list: Vec<f32> = vec![2.5, 9.2, 123.45, 42.0];
    let result: f32 = largest(&number_list);
    println!("The largest number is {}", result);
}
```

But what if we wanted to pass in a list containing both `i32` and `f32`?

```rust
fn main() {
    let number_list = vec![34, 2.5, 123.45, 50]; // Compile Error!
}
```

Since a vector **must** be homogenous, and thus does not allow mixing data types, this does not compile.
Even a generic is not able to solve this.

But enums are!

```rust
enum Number {
    Int(i32),
    Float(f32),
}

fn main() {
    let number_list: Vec<Number> = vec![
        Number::Int(34),
        Number::Float(2.5),
        Number::Float(123.45),
        Number::Int(50),
    ];
}
```

The Vector is technically still homogenous, as every element is of the type `Number`

### Exercise

Exercise: Implement the `largest` function using our new `Number` enum.

> [!NOTE]
> To simplify the task, we are not using references here.

```rust
fn largest(list: Vec<Number>) -> Number {
    // ...
}
```

> [!TIP]
> Instead of „moving“ a `Number`, we always want to „copy“ it instead.
> We can tell Rust to do so by annotating the `Number` enum with `#[derive(Clone, Copy)]`.
>
> In order to use the `Number` enum inside `println!` we also need to annotate it with `#[derive(Debug)]`.
>
> ```rust
> #[derive(Clone, Copy, Debug)]
> enum Number {
>     Int(i32),
>     Float(f32),
> }
> ```

> [!TIP]
> To convert an `i32` into a `f32` you can use the `as` cast.
>
> ```rust
> let a: i32 = ...;
> let b: f32 = a as f32;
> ```

### Bonus Exercise

Implement the `largest` function using references:

```rust
fn largest(list: &[Number]) -> &Number {
    // ...
}
```

## Pattern Matching in `let ... else` statements

The `let ... else` statement tries to match a given pattern `...`.
If it fails to match, it executes the `else` branch.

> [!NOTE]
> The `else` branch must diverge, meaning it must contain a statement interrupting the control flow, e.g. `return` or `panic!()`

```rust
fn analyse_circle(shape: &Shape) {
    let Shape::Circle(radius) = shape else {
        println!("Not a circle.");
        return;
    };

    println!("The circle has the radius {}", radius);
    println!("Its area is {}", 3.14 * (radius ^ 2) as f32)
}
```

[^1]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#defining-modules-to-control-scope-and-privacy
