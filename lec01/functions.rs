fn main() {
    let z = empty();
    println!("{:?}", z);
    parameter(1, 2);
    let z = retval(2, 2);
    println!("{}", z);

    print!("{}", max(10, 20));
}

fn empty() {}

fn parameter(x: i32, y: i32) {
    let x = println!("{} {}", x, y);
    println!("{:?}", x);
}

fn retval(x: i32, y: i32) -> i32 {
    let z = x + y;
    // return z;
    // z
    x + y
}

fn max(x: i32, y: i32) -> i32 {
    let z = if x > y { x } else { y };
    z
}
