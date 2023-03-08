#[derive(Debug)]
enum Something {
    Int(i32),
    Float(f64),
}

fn vector_usage() {
    let mut v: Vec<i32> = Vec::new();
    v.fill(1);
    v.push(2);

    let first: &i32 = &v[0];
    println!("first element is {first}");

    let first: Option<&i32> = v.get(0);
    match first {
        Some(first) => println!("first element is {first}"),
        None => println!("no first element"),
    }

    for i in &mut v {
        println!("get {i}");
        *i += 1;
    }

    // easier way
    let mut v2 = vec![1, 2];
    v2[0] = 1;

    let sth = vec![Something::Int(1), Something::Float(1.1)];
    println!("first element is {:?}", sth[0]);
}

fn main() {
    vector_usage();
}
