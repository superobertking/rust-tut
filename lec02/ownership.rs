#[derive(Debug, Clone)] // cannot implement Copy for it
struct Student {
    name: String,
    id: StudentID,
}

/// Copy can only be implemented if the type has !Drop.
#[derive(Debug, Clone, Copy)]
struct StudentID {
    id: i32,
}

#[derive(Debug)]
struct File {
    fd: i32,
}

impl Drop for File {
    fn drop(&mut self) {
        // do nothing
        println!("File is dropped");
    }
}

fn main() {
    /* struct String {
        data: Vec<u8>,
    }
    struct Vec {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }
    impl Drop for String {
        fn drop(&mut self) {
            delete self.ptr;
        }
    } */

    let h = String::from("hello");
    // let mut h2 = h;
    let mut h2 = h.clone();
    h2.push_str(", world");

    println!("{}", h);
    println!("{}", h2);

    // scalar types are always Copy
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let stid = StudentID { id: 10 };
    let stid2 = stid;
    println!("{:?} {:?}", stid, stid2);

    let st = Student {
        name: String::from("Robert"),
        id: StudentID { id: 10 },
    };

    let st2 = Student {
        name: st.name.clone(),
        id: st.id.clone(),
    };
    let st3 = Student {
        name: st.name.clone(),
        id: st.id,
    };
    let st4 = st.clone();
    // on stack
    let st5 = st;
    println!("{:?} {:?} {:?} {:?}", st2, st3, st4, st5);
    // println!("{:?}", st); // error

    // Clone and Drop are implemented for types with heap-allocated resources
    // like String and Vec
    let v = vec![1, 2, 3];
    dbg!(&v);
    let v_clone = v.clone();
    dbg!(&v_clone);
    let v_move = v; // This is a move,
                    // and doing `dbg!(v);` again will be a compile error
    dbg!(&v_move);

    // drop(st5);
    mydrop(st5);
    // println!("{:?}", st5); // error
    mydrop2(stid);
    println!("{:?}", stid);

    // Ownership could also be used to express uniqueness of some value.
    {
        let owned_file = File { fd: 1 };
        let _moved_file = owned_file;
        // owned_file will no longer be accessible
    }
    // _moved_file will no longer be accessible
    let _ = File { fd: 1 };
    // be careful: file dropped immediately
    println!("End of file block");

    // Boxed values have Drop trait
    let boxed = Box::new(5);
    let boxed_move = boxed;
    println!("{}", boxed_move);
    let boxed_cloned = boxed_move.clone();
    println!("{}", boxed_cloned);
    // boxed_move is moved here, while boxed_clone is still alive
}

fn mydrop(_st: Student) {}

fn mydrop2(_st: StudentID) {}
