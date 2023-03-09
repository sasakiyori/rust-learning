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

fn string_usage() {
    let s1 = "abc";
    let mut s2 = "abc".to_string();
    let mut s3 = String::from("abc");

    s2.push('x');
    s3.push_str(" append");
    s3.push_str(s1);
    // push_str uses the &str, so the ownership of 's1' still exist
    println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    // concat string:  fn add(self, s: &str) -> String {
    // 首先，s2 使用了 &，意味着我们使用第二个字符串的 引用 与第一个字符串相加。
    // 这是因为 add 函数的 s 参数：只能将 &str 和 String 相加，不能将两个 String 值相加。
    // 不过等一下 —— 正如 add 的第二个参数所指定的，&s2 的类型是 &String 而不是 &str。
    // 那为什么还能编译呢？
    // 之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。
    // 当add函数被调用时，Rust 使用了一个被称为 Deref 强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。
    // 因为 add 没有获取参数的所有权，所以 s2 在这个操作后仍然是有效的 String。
    let s4 = s3 + &s2;
    println!("s2 is {s2}");
    println!("s4 is {s4}");

    // format() doesn't affect ownership
    let s5 = format!("combine-{s2}-{s4}");
    println!("s2 is {s2}");
    println!("s4 is {s4}");
    println!("s5 is {s5}");
}

fn main() {
    vector_usage();
    string_usage();
}
