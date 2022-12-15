fn main() {
    println!("Hello, world!");

    let x = 1;
    let y = 'y';

    // x: 1, y: y
    println!("x: {}, y: {}", x, y);

    // x: 1, x: 1
    println!("x: {0}, x: {0}", x);

    // x: 1, x: 1, y: y
    println!("x: {0}, x: {0}, y: {1}", x, y);
    // invalid: 'y' argument never used
    // println!("x: {0}, x: {0}, y: {}", x, y);

    // escape: {,}
    println!("escape: {{,}}");
}
