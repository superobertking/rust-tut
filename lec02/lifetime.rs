fn main() {
    // 'outer
    /* let r;
    {
        // 'inner
        let x = 5;
        r = &x;
    }
    println!("{}", r); */
    let t = {
        let x = 1;
        bar(&x)
    };
    let t = {
        // 'x
        let x = 1;
        // 'x
        let z;
        {
            // 'y
            let y = 10;
            // 'x
            z = foo(&x, &y);
        }
        println!("{}", z);
        bar(&x)
    };

    let x = 10;
    let t = ttt(&x);

    let st = Student {
        name: "Robert",
        id: 10,
    };
    println!("{:?}", st);

    /* let h = String::from("Robert");
    let st = Student { name: &h, id: 10 };
    // drop(h); // error
    println!("{:?}", st); */
}

fn ttt(x: &i32) -> &i32 {
    x
}

fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

fn bar<'a>(x: &'a i32) -> &'static i32 {
    // x // error
    &1
}

#[derive(Debug, Clone)] // cannot implement Copy for it
struct Student {
    name: &'static str,
    id: i32,
}

/* #[derive(Debug, Clone)] // cannot implement Copy for it
struct Student<'a> {
    name: &'a str,
    id: i32,
} */

// | main     | &x
// | print    | (x) i

/* fn dangling() -> &String {
    let x = String::from("hello");
    &x

    // &String::from("hello")
} */

/* You don't declare lifetimes. Lifetimes come from the shape of your code,
 * so to change what the lifetimes are, you must change the shape of the code.
 * -- https://users.rust-lang.org/t/lifetime-of-a-returned-iterator/43732/2 */
