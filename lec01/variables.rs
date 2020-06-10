const GLOBAL_X: i32 = 2;
static GLOBAL_X_S: i32 = 1;

fn f() {
    static mut LOCAL_X_S: i32 = 1;
    unsafe {
        LOCAL_X_S += 1;
        dbg!(LOCAL_X_S);
    }
}

fn main() {
    static mut LOCAL_X_S: i32 = 1;

    // const auto x = 1;
    let mut x = 1;
    println!("{}", x);
    x = 2;
    println!("{}", x);

    let (y, z) = (10, 11);
    println!("y is {}, z is {}", y, z);

    const LOCAL_X: i32 = 1;
    dbg!(LOCAL_X);
    dbg!(GLOBAL_X);
    unsafe {
        dbg!(GLOBAL_X_S);
        dbg!(LOCAL_X_S);
    }
    f();
    f();
}
