# Structs

## Recap

1. Ownership and Ownership rules
2. Scopes (`{}`)
3. Immutable References (`&my_variable`)
4. Mutable References (`&mut my_variable`)

## Heap vs. Stack

[^1]

Many programming languages don’t require you to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.

### Stack

The stack stores values in the order it gets them and removes the values in the opposite order.
This is referred to as _last in, first out_.

> [!TIP]
> Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top.
> Adding or removing plates from the middle or bottom wouldn’t work as well!

> [!IMPORTANT]
> All data stored on the stack must have a known, fixed size.
> Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

### Heap

The heap is less organized: when you put data on the heap, you request a certain amount of space.
The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.

This process is called _allocating_ (pushing values onto the stack is not considered allocating).
Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

> [!TIP]
> Think of being seated at a restaurant. When you enter, you state the number of people in your group,
> and the host finds an empty table that fits everyone and leads you there.
> If someone in your group comes late, they can ask where you’ve been seated to find you.

---

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

[^1]: (https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)

## Structs

See [Slides](https://github.com/pfhaupt/progkurs/blob/794c7d4916789ae00e5000b7a3ad4ffbb6f48939/rust-beginner/08%20-%20Structs/08%20-%20RUSTikales%20Rust%20for%20beginners.pptx) by [Philippe Felix Haupt](https://github.com/pfhaupt).

## Exercise 1

Create a program that prints a rectangle.

A rectangle should be represented as a struct and provide two functions:

1. an **associated function** `new`, that constructs a rectangle from a given `width` and `height`.
2. a **method** `print`, that prints the rectangle to the console using hashtags (`#`).

**Example**

```rust
fn main() {
    let my_rectangle = Rectangle::new(10, 3);
    my_rectangle.print();
}
```

**Output**

```rust
##########
##########
##########
```

> [!TIP]
> You can use the range syntax (e.g. `1..6`) with variables:
>
> **Example**
>
> ```rust
> let start = 5;
> let stop = 10;
> for i in start..stop {
>     println!("{}", i);
> }
> ```
>
> **Output**
>
> ```console
> 5
> 6
> 7
> 8
> 9
> ```

> [!TIP]
> You may want to use `print!` instead of `println!`.
> `print!` is equivalent to `println!`, except that a line break is **not** printed at the end of the message.
>
> **Example**
>
> ```rust
> print!("#")
> print!("#")
> print!("#")
> ```
>
> **Output**
>
> ```console
> ###
> ```

> [!TIP]
> You can create a line break using either `print!("\n")` or `println!()`.

## Exercise 2

Modify your program, so the rectangle can be positioned at any `x` and `y` coordinates.
Empty space should be represented by dots (`.`).

**Example**

```rust
fn main() {
    let my_rectangle = Rectangle {
        position: Point { x: 5, y: 2 },
        width: 10,
        height: 3,
    };
    my_rectangle.print();
}
```

**Output**

```rust
...............
...............
.....##########
.....##########
.....##########
```
