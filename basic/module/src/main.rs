mod a {
    /*
    mod a1 {
        fn a_11() {
            println!("A-A1-A11");
        }
    }

    mod a2 {
        pub fn a_22() {
            println!("A-A2-A22");
        }
    }

    pub mod a3 {
        fn a_33() {
            println!("A-A3-A33");
        }
    }
    */

    pub mod a4 {
        pub fn a_44() {
            println!("A-A4-A44");
        }
    }
}

fn main() {
    println!("Hello, world!");
    // fail.
    //     a::a1::a_11();
    //        ^^ private module

    // fail.
    //     a::a2::a_22();
    //        ^^ private module

    // fail.
    //     a::a3::a_33();
    //            ^^^ private function

    // success.
    a::a4::a_44();
}
