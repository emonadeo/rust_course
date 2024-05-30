> [!IMPORTANT]
> This file is a draft and subject to major changes.

# Loops and Vectors

## Recap

1. Loops: `while`, `loop`
2. Loop Control Flow: `break`, `continue`
3. Lists: Arrays (Fixed Length), Vector (Variable Length)
4. Extending the Guessing Game to allow multiple guesses and store the guesses in a `Vec`.

## Functions

With functions you can define reusable code blocks, i.e. a set of instructions.

You’ve already seen one of the most important functions in the language:
the `main` function, which is the entry point of many programs.

You’ve also seen the `fn` keyword, which allows you to declare new functions.

Similar to variables, every function needs to have a name that follows the `fn` keyword.

Both variable names and function names must only consist of _underscores_ and _alphanumeric characters_,
which means lowercase letters `a-z`, uppercase letters `A-Z` and digits `0-9`.

Then, add a set of parantheses `()`. (What these are for is discussed later)

Finally use curly braces `{}` that contain the instructions we want the function to execute **when called**.

```rust
fn my_function() {
    println!("Print this.");
    println!("Print that.");
    let some_number = 2;
    let another_number = some_number * 200;
    println!("The result is: {}", some_number);
}
```

By itself, a function definition does nothing. (except for the special `main` function of course)  
In order to run the defined instructions, we need to **call** the function.

To call a function, we use the name and parantheses.

> [!NOTE]
> Do not forget the semicolon, that terminates every instruction.

```rust
// define the function here
fn another_function() {
    println!("Another function.");
}

fn main() {
    println!("Hello World!");

    another_function(); // <- call the function here
}
```

**Output**

```console
Hello World!
Another function.
```

### Parameters

**Parameters** are used to pass values into a function.
We can define functions to have **parameters**, which are special variables that are part of a function’s signature.

When a function has **parameters**, you can provide it with concrete values for those parameters.
Technically, the concrete values are called **arguments**, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

```rust
fn greet(name: String) { // <- `name` is a parameter of type `String`
    println!("Hello {}.", name);
}

fn main() {
    let my_name = "Emanuel".to_string();
    greet(my_name); // <- `my_name` is the argument

    let your_name = "Max".to_string();
    greet(your_name); // <- `your_name` is the argument
}
```

**Output**

```console
Hello Emanuel.
Hello Max.
```

We can also define multiple parameters by separating them with commas

```rust
fn greet(name: String, age: u16) {
    println!("Hello {}.", name);
    println!("You are {} years old.", age);
}

fn main() {
    let my_name = "Emanuel".to_string();
    let my_age = 23;
    greet(my_name, my_age);
}
```

**Output**

```console
Hello Emanuel.
You are 23 years old.
```

### Return Values

We learned, that **parameters** are used to pass values **into** a function.  
Complimentary, **return values** are used to pass values **out of** a function.

We don’t name return values, but we must declare their type after an arrow (`->`).
To return a value, use the `return` keyword, followed by any value.

Function calls, that return a value, **are a value itself**.
This means, we can use the function call just like any other value, e.g. assigning them to a variable or printing it using `println!()`.

```rust
fn square(number: i64) -> u64 {
    return number * number as u64;
}

fn main() {
    let my_number = 12;
    let my_number_squared = square(my_number);
    println!("{} squared is {}.", my_number);
    // or
    println!("{} squared is {}.", 12, square(12));
}
```

**Output**

```console
12 squared is 144.
```

#### Early Returns

One very important aspect about `return` statements is that any instructions defined after a `return` statement **do not** get executed.
Returning a value means the function is done.
When a running program encounters a `return` statement, it immediately exits the function and jumps back to where it was originally called.

```rust
fn square(number: i64) -> u64 {
    return number * number as u64;
    println!("This never ever gets printed! :(");
}

fn main() {
    println!("{} squared is {}.", 12, square(12));
}
```

**Output**

```console
12 squared is 144.
```

This also applies for loops, exiting both the loop (like a `break` statement), and the function altogether:

```rust
fn square(number: i64) -> u64 {
    while true {
        return number * number as u64;
        println!("This never ever gets printed! :(");
    }
}

fn main() {
    println!("{} squared is {}.", 12, square(12));
}
```

**Output**

```console
12 squared is 144.
```

#### Bonus: The Guard Pattern

We can combine early returns with if statements for powerful control flow without the use of `else`.
You will come across this pattern very often.

```rust
fn absolute(number: i64) -> u64 {
    if (number < 0) {
        return number * -1 as u64;
    }
    return number; // <- this only gets executed, if number >= 0
}

fn main() {
    let a = absolute(-4); // returns 4
    let b = absolute(6); // returns 6
}
```

See: https://en.wikipedia.org/wiki/Guard_(computer_science)

### Exercise

TODO
