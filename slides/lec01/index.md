class: center, middle

# Introduction to Rust
## 01: All The Basics
Yuzhuo Jing

RUShanghaiTech x Geek Pie

2020.06.??

---

# Before we start

## Why having this talk?

In Rust [survey](https://blog.rust-lang.org/2020/04/17/Rust-survey-2019.html) 2019: (rephrased)

> *People are asking for more beginner and intermediate level learning material about Rust, a lot of which asked for video content specifically.*

This talk series aims to give a thorough introduction to Rust, not just an overview of the language.

## Prerequisite

- Basic knowledge about at least one language in Python/C/C++ is preferred.

---

# Agenda

1. Installation

2. hello, world

3. Variables & Mutability

4. Data Types

5. Control Flow

6. Functions

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

Single line: `// ...`

doc: `/// ...` we will mention in `cargo`

block comment: `/* ... */`

---

layout: false
layout: true

# Variables & Mutability

---

### Immutable & Mutable Variables
```
let x = 1;
```

Shadowing

---

### Constants

---

### Static

---

layout: true

# Data Types

---

### Primitive Types

- Scalar Types: Integer, Floating-Point Types, (Numeric Operations), Boolean, Character
- Compound Types: Tuple, Array (Array access)

---

### Structs

---

### Enums

---

layout: false
layout: true

# Control Flow

TODO `scope`: also explain in the next talk

---

### `if`

`else` and `else if`

also an expression: `let x = if cond {} else {}`

---

### `loop`

---

### `while`

---

### `for`

---

exclude: true

### `match`

pattern matching

---

layout: false
layout: true

# Functions

---

### Parameters

---

### Body

return value as a expression

---

layout: false


# Summary

1. Statements

2. Variables & Mutability

3. Data Types
    - Primitive types, Structs, Enums

4. Control Flow
    - `if`, `loop`, `while`, `for`, `match`, `let`

5. Functions

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


