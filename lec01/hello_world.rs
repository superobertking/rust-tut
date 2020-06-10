/// Let's start writing the very first program in Rust!
///
/// Then let's compile with `rustc`:
///
/// ```bash
/// $ rustc hello_world.rs
/// ```
///
/// And run it:
/// ```
/// $ ./hello_world
/// hello, world
/// ```
///
/// This block, starting with `///` in each line, is a doc comment.
/// Try this out:
/// ```
/// $ rustdoc hello_world.rs
/// ## Then open doc/hello_world/index.html to view the docs.
/// ```
///
/// You can also use
/// ```
/// $ rustup doc
/// $ rustup doc --std
/// $ rustup doc --book
/// ```
/// to view Rust official documentation.
pub fn main() {
    // this is a comment
    println!("hello, world");
    /* this is a block comment
     *
     *  asdfasdfa
     *
     * */
}
