fn main() {
    // if (..) {...}
    // if cond:
    //      ...

    let (x, y) = (1, 2);
    if x == y {
        println!("they are the same");
    } else {
        println!("they are different");
    }

    #[derive(Debug, PartialEq)]
    enum CEnum {
        Red = 0,
        Green,
        Blue = 100,
    }
    let r = CEnum::Red;

    if r == CEnum::Red {
        println!("it is red");
    } else if r == CEnum::Green {
        println!("it is green");
    } else {
        println!("it is blue");
    }
    // match is better

    let mut ty = 100u32;
    while ty != 0 {
        print!("{}", ty & 1);
        ty >>= 1;
    }
    println!();

    let mut ty = 100u32;
    // while(true)
    loop {
        if ty == 0 {
            break;
            // continue;
        }
        print!("{}", ty & 1);
        ty >>= 1;
    }
    println!();

    // for i in range(0, 10):

    // for i in 0..10 { // [0,10)
    for i in 0..=10 {
        // [0,10]
        print!("{} ", i);
    }
    println!();

    let arr = [4, 3, 12, 3];
    // for i in 0..arr.len()+1 { // will panic
    for i in 0..arr.len() {
        print!("{} ", arr[i]);
    }
    println!();

    // this is better
    for x in &arr {
        print!("{} ", x);
    }
    println!();

    let arr = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    'outer: for subarr in &arr {
        for x in subarr {
            println!("testing x = {}, we need 7", x);
            if x == &7 {
                break 'outer;
            }
        }
    }

    let mut x = 1012;
    let resx = 'outer: loop {
        let mut flag = true;
        'inner: for i in 2..x {
            if x % i == 0 {
                flag = false;
                break 'inner;
            }
        }
        if flag {
            break 'outer x;
        }
        x += 1;
    };
    println!("resx = {}", resx);
}
