#[derive(Debug)]
struct X {
    a: i32,
    b: i32,
}

fn main() {
    let x = X { a: 1, b: 2 };
    println!("{}, {}", x.a, x.b);
    // X { a: 1, b: 2 }
    println!("{:?}", x);
    /*
        X {
            a: 1,
            b: 2,
        }
    */
    println!("{:#?}", x);
    /*
        [src/main.rs:11] &x = X {
            a: 1,
            b: 2,
        }
    */
    dbg!(&x);
}
