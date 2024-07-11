# Generics & Traits

## Slices

[^1]

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

```rust
let s: String = String::from("hello world");

let hello: &str = &s[0..5];
let world: &str = &s[6..11];
```

Rather than a reference to the entire string, `hello` is a reference to a portion of the string, specified in the extra `[0..5]` bit.
We create slices using a range within brackets by specifying `[starting_index..ending_index]`,
where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice.

<img src="assets/07_slices.svg" width="300" />

With Rust’s `..` range syntax, if you want to start at index `0,` you can drop the value before the two periods.
In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

Likewise, if your slice includes the last byte of the string, you can drop the trailing number.
That means these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

Finally, you can also drop both values to take a slice of the entire string.
So these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

## Generics

### Generics in Functions

[^2]

Consider the following scenario: We want to create a function that finds and returns the largest number in a given list of numbers.

The resulting function may look like this for example:

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

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
}
```

However this only works for lists of `i32` numbers.
What about other numeric data types like `u32`, or `f32`?
We could convert between these data types, but then we may lose information (`f32 -> i32`) or run into errors (`i32 -> u32`).

So our best option would be to define a new function doing the exact same, except for `f32` instead of `i32`.

```rust
fn largest_f32(list: &[f32]) -> &f32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
```

```rust
fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list: Vec<f32> = vec![2.5, 9.2, 123.45, 42.0];
    let result = largest_f32(&number_list);
    println!("The largest number is {}", result);
}
```

The function bodies have the same code, so let’s eliminate the duplication by introducing a **generic type parameter** in a single function.

To parameterize the types in a new single function, we need to name the type parameter, just as we do for the value parameters to a function.
You can use any identifier as a type parameter name.
But we’ll use `T` because, by convention, type parameter names in Rust are short, often just a letter, and Rust’s type-naming convention is UpperCamelCase.
Short for “type,” `T` is the default choice of most Rust programmers.

When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what that name means.
Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.
To define the generic largest function, place type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list, like this:

```rust
fn largest<T>(list: &[T]) -> &T {
    // ...
}
```

We read this definition as: the function `largest` is generic over some type `T`.
This function has one parameter named `list`, which is a slice of values of type `T`.
The function `largest` will return a reference to a value of the same type `T`.

Fully implemented the new generic `largest` function looks like this:

> [!NOTE]
> This code won’t compile yet, but we’ll fix it later.

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list: Vec<f32> = vec![2.5, 9.2, 123.45, 42.0];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

### Generics in Structs

We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax.
For example, we can define a `Point<T>` struct to hold `x` and `y` coordinate values of any type:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

```rust
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

In this case `x` and `y` are the **same** type, as they are both typed as `T`.
If we try to create a Point where `x` and `y` have different types, our code will not compile:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

To define a Point struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

> [!NOTE]
> You can also have multiple generic types in functions:
>
> ```rust
> fn do_something<T, U>(x: T, y: U) {
>     // ...
> }
> ```

### Generics in Methods

We can implement methods on structs (and later enums) and use generic types in their definitions, too.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

```rust
fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

> [!NOTE]
> We have to declare `T` just after `impl` so we can use `T` to specify that we’re implementing methods on the type `Point<T>`.
> By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type.

We can also specify constraints on generic types when defining methods on the type.
We could, for example, implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type.

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

This code means the type `Point<f32>` will have a `distance_from_origin` method.
Other instances of `Point<T>` where `T` is not of type `f32` will **not** have this method defined.

---

Generics are very powerful.
Using both generics in `impl` and in functions we can achieve the following:

```rust
struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    //            X1   Y1
    let p1: Point<i32, f32> = Point { x: 5, y: 10.4 };
    //            X2   Y2
    let p2: Point<&str, char> = Point { x: "Hello", y: 'c' };

    //            X1   Y2
    let p3: Point<i32, char> = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

## Traits

See [Slides](07_traits_slides.pdf) by [Philippe Felix Haupt](https://github.com/pfhaupt).

### The `From` and `Into` Trait

```rust
struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> From<(X, Y)> for Point<X, Y> {
    // „Convert from a Tuple into a Point“
    // (NOT vice versa)
    fn from(value: (X, Y)) -> Self {
        return Point {
            x: value.0,
            y: value.1,
        };
    }
}

fn main() {
    let my_tuple: (i32, i32) = (2, 5);
    // using `from` directly...
    let my_point: Point<i32, i32> = Point::from((2, 5));
    println!("x: {}, y: {}", my_point.x, my_point.y);
    // or using `into`..
    let my_point: Point<i32, i32> = my_tuple.into();
    println!("x: {}, y: {}", my_point.x, my_point.y);
}
```

> [!NOTE]
> Quiz: With the code above, we can convert tuples into points.
> But how can you implement the conversion from points into tuples?
> (i.e. `Point<X, Y> -> (X, Y)`)

### The `AsRef` Trait

TODO

[^1]: https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type
[^2]: (https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions)
