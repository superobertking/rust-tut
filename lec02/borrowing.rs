fn main() {
    let h = String::from("hello");
    // let h2 = append_world(&h);
    // append_world_move(h);
    let mut h2 = h.clone();
    append_world_mut(&mut h2);
    println!("{} {}", h, h2);

    let mut h = String::from("hello");

    // {
    let h1 = &h;
    let h2 = &h;
    /* let h3 = &mut h; // error here
    h3.push_str(", world"); */
    // h1 -> h?
    println!("{} {}", h1, h2);
    // }
    let h3 = &mut h;
    h3.push_str(", world");

    let _ = append_world("hello");
}

fn append_world_mut(s: &mut String) {
    s.push_str(", world");
}

// fn append_world(s: &String) -> String {
fn append_world(s: &str) -> String {
    dbg!(std::mem::size_of::<&str>());
    dbg!(std::mem::size_of::<String>());

    let s: &'static str = "hello";
    String::from("hello");

    // slice: &[T]
    // [T; N]

    /* #[derive(Clone, Copy)]
    struct SliceRepr<T> {
        ptr: *const T,
        len: usize,
    } */
    #[derive(Clone, Copy)]
    struct StrRepr {
        ptr: *const u8,
        len: usize,
    }
    union S<'a> {
        repr: StrRepr,
        st: &'a str,
    }

    unsafe {
        let repr = S { st: s }.repr;
        println!("{:?} {}", repr.ptr, repr.len);
        println!("{:?} {}", s.as_ptr(), s.len());
    }

    s.to_owned() + ", world"
    // A more efficient way to do string concatenation is by using
    // [s, ", world"].concat()
}

fn append_world_move(s: String) -> String {
    s + ", world"
}
