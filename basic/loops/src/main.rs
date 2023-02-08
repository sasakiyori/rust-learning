fn main() {
    // 1 2
    for a in 1..3 {
        println!("{a}");
    }
    println!("---------");
    // 1 2 3
    for b in 1..=3 {
        println!("{b}");
    }
    println!("---------");
    // 2 1
    for c in (1..3).rev() {
        println!("{c}");
    }
}
