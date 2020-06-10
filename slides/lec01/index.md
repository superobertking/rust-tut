class: center, middle

# Introduction to Rust
## 01: All The Basics
Yuzhuo Jing

RUShanghaiTech x Geek Pie

2020.06.07

---

# Before we start

## Why having this talk?

In Rust [survey](https://blog.rust-lang.org/2020/04/17/Rust-survey-2019.html) 2019: (rephrased)

> *People are asking for more beginner and intermediate level learning material about Rust, a lot of which asked for video content specifically.*

This talk series aims to give a thorough introduction to Rust, not just an overview of the language.

## Prerequisite

- Basic knowledge about at least one language in Python/C/C++.

---

# Agenda

1. Installation

2. hello, world

3. Variables & Mutability

4. Data Types

5. Functions

6. Control Flow

---

# Installation

The recommended way of installing Rust toolchain:

Go to [https://rustup.rs](https://rustup.rs) and get started.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

layout: true

# hello, world

---

We always start with the good-old way of learning a language. :)

In `hello_world.rs`:
```
fn main() {
    println!("hello, world");
}
```

And then compile it with `rustc`:
```bash
$ rustc hello_world.rs
```
And run it:
```
$ ./hello_world
hello, world
```

.mygrey[*We will start with single file compilation for now, and bring up `cargo` later.*]

---

### Comments

```
//! This is a documentation for this file

/// This a documentation comment
fn main() {
    // This is a single-line comment
    println!("hello, world");
    // println!("this statement is commented");

    /* This is a block comment. */
    /** Outer block doc with /*! Inner block doc */ */
    let x = 1;
}
```

---

layout: false
layout: true

# Variables & Mutability

---

### Immutable & Mutable Variables
```
fn main() {
    // Immutable variable (by default)
    let x = 1;
    // Mutable variable
    let mut y = 2;
    // Assign again to variable `y`
    y = 5;

    // Shadowing occurs: old `x` is shadowed (but may not be dropped).
    let x = x * y;
    let y = y;  // Rebind (freezed) as immutable
    y = 10; // Error here

    // Variable with type annotation.
    // If no type specified, it will be inferred.
    let z: i32 = x + y;
}
```

---

### Constants

```
/// You must annotate the type,
const GLOBAL_X: i32 = 1;

fn main() {
    // and the naming convention for constants
    // in Rust should be all-uppercase.
    const LOCAL_Y: i32 = 2;
}
```

???
`const fn`

---

### Static

```
/// Global static variables
static GLOBAL_X: i32 = 0;

fn f() {
    // Local static variables
    static mut LOCAL_Y: i32 = 0;

    // Accessing is unsafe because of potential data race.
    // We will talk about unsafe later.
    unsafe {
        LOCAL_Y += 1;
        dbg!(LOCAL_Y);
    }
}
```

---

layout: false
layout: true

# Data Types

### Primitive Types

---

#### Scalar types

- Integers: `{i,u}{8,16,32,64,128,size}`
- Floating-point: `f{32,64}`
- Character: `char` (4 bytes Unicode)
- Boolean: `bool` (1 byte)

#### Operators

- Logical: `!`, `!=`, `==`, `<`, `<=`, `>`, `>=`, `&&`, `||`
- Numeric (Arithmetic): `%`, `*`, `/`, `+`, `-`, `<<`, `>>`, `^`, `&`, `|`
- Assignment: `=`, `%=`, `*=`, `/=`, `+=`, `-=`, `<<=`, `>>=`, `^=`, `&=`, `|=`

---

#### Compound types: Tuple & Array

```
fn main() {
    let arr = [1, 2, 3];    // Array with 3 elements
    let arr = [0; 5];       // Array of 5 zeros
    dbg!(arr[3]);

    let tup = (1, true, 'A');   // Tuple with any number of any type.
    dbg!(tup.1);
}
```

---

layout: false
layout: true

# Data Types

---

### Structs

```
struct Point2D {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point2D { x: 1, y };
    println!("The point is {{ x = {}, y = {} }}", point.x, point.y);
}
```

---

### Enums

```
enum Fruit {
    Apple,
    Orange(i32, char),
    Banana { color: char, weight: i32 },
}
```

---
layout: false
layout: true

# Functions

---
### Parameters & Return Type

```
fn mymax(x: i32, y: i32) -> i32 {
```

---
### Body

```
fn mymax(x: i32, y: i32) -> i32 {
    if x == 0 || y == 0 {
        return -1;  // early return in statement
    }

    // return as expression
    if x > y {
        x
    } else {
        y
    }
}
```

---
layout: false
layout: true

# Control Flow

---
### `if`

```
fn main() {
    let (x, y) = (1, 2);
    if x == y {
        println!("they are the same");
    } else if x > y {
        println!("x is larger than y");
    } else {
        println!("they are different");
    }
}
```

???

TODO `scope`: also explain in the next talk

---

### `while`

```
let mut ty = 100u32;
while ty != 0 {
    print!("{}", ty & 1);
    ty >>= 1;
}
println!();
```

---

### `loop`

```
let mut ty = 100u32;
loop {
    if ty == 0 {
        break;
        // or in other cases use `continue;`
    }
    print!("{}", ty & 1);
    ty >>= 1;
}
println!();
```

---
### `for`

```
for i in 0..=10 { // [0,10]
    for j in 0..10 { // [0,10)
        print!("{} {}, ", i, j);
    }
    println!();
}
```

```
let arr = [4, 3, 12, 3];
for x in &arr {
    print!("{} ", x);
}
println!();
```

---
### Loop labels

```
let arr = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
'outer: for subarr in &arr {
    for x in subarr {
        println!("testing x = {}, we need 7", x);
        if x == &7 {
            println!("We found 7!");
            break 'outer;
        }
    }
}
```

---
### `match`

```
#[derive(Debug)]
enum MyOption {
    MySome(i32),
    MyNone,
}

let op = MyOption::MySome(1);

match op {
    MyOption::MySome(x) => println!("This is a MySome of {:?}", x),
    MyOption::MyNone => println!("This is a None"),
}
```

???
TODO: talk about if let, while let

---

layout: false

# Summary

1. Statements

2. Variables & Mutability

3. Data Types
    - Primitive types, Structs, Enums

4. Functions

5. Control Flow
    - `if`, `while`, `loop`, `for`, `match`

---

layout: false

# After this talk

Now you should be able to write Rust as C (without pointer & bit fields).

Inarguably, C without pointer is soulless (莫得灵魂).

Next talk we will talk about The Three Swords: Ownership, Borrowing and Lifetime.

---
class: center, middle

# Thanks for listening!
Yuzhuo Jing

RUShanghaiTech x Geek Pie


