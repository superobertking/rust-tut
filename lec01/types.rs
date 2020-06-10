fn main() {
    use std::mem::{size_of, size_of_val};

    // i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    let /* mut */ x: i32 = 2; // 000...010
    let yi: i32 = -x; // 111...110
    let yu: u32 = -x as u32; // 111...110

    println!("{}", size_of_val(&x));
    println!("{}, {}, {}", x, yi, yu);
    println!("{:x}, {:x}, {:x}", x >> 1, yi >> 1, yu >> 1);
    // x86: shr, sar
    // mips/riscv: srl, sra

    let mut ty = yu;
    while ty != 0 {
        print!("{}", ty & 1);
        ty >>= 1;
    }
    println!();

    // We can shadow previous variable with the same name!
    let x: u32 = 0;
    dbg!(x);
    // dbg!(x - 1);
    dbg!(x.wrapping_sub(1));
    dbg!(x.checked_sub(1));
    // This is a Option::None; you may temporarily think of Option as (bool, val).

    let c: char = '你';
    println!("{}", size_of_val(&c));
    let s = "㊗️";
    println!("{}", s);
    println!("{}", s.len());

    let c: u8 = b'A'; // This is different from C char.
    let s: &[u8; 12] = b"hello, world"; // This is different from C string.

    // let s1 = b"你好，世界"; // error

    let tuple: (i32, i32, i32) = (1, 2, 3);
    let tuple: (i32, char, bool) = (1, '早', false);
    println!("{:?}", tuple);
    // __str__ __repr__
    println!("({:?}, {:?}, {:?})", tuple.0, tuple.1, tuple.2);

    #[derive(Debug)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point3D { x: 1, y: 2, z: 3 };
    println!("{:?}", point);
    println!("{:#?}", point);

    point.x = 100;
    println!("{:?}", point);

    #[derive(Debug)]
    struct NewPoint3D {
        x: i32,
        y: char,
        z: MyBool,
    }

    #[derive(Debug)]
    struct MyBool {
        b: bool,
    }

    let point = NewPoint3D {
        x: 1,
        y: '早',
        z: MyBool { b: true },
    };
    println!("{:?}", point);

    struct ZeroSized {}
    dbg!(size_of::<ZeroSized>());
    // type state

    #[derive(Debug)]
    enum MyOption {
        MySome(i32),
        MyNone,
    }

    let op = MyOption::MySome(1);
    // let op = MyOption::MyNone;
    println!("{:?} size is {}", op, size_of::<MyOption>());

    match op {
        MyOption::MySome(x) => println!("{:?}", x),
        MyOption::MyNone => println!("this is a none"),
    }

    #[derive(Debug)]
    enum CEnum {
        Red = 0,
        Green,
        Blue = 100,
    }
    let _ = CEnum::Red;
}
