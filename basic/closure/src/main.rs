use std::thread;

fn borrow_immutable_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn borrow_mutable_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(4);
    // after defining a mutable closure, the variable has been already borrowed

    // so cannot borrow `list` as immutable because it is also borrowed as mutable by closure!
    // print function will cause error:
    // println!("When capture {:?}", list);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn move_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

fn main() {
    borrow_immutable_closure();
    borrow_mutable_closure();
    move_closure();
}
