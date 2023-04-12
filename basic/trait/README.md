## trait不同的写法和对应不同的功能
```rust
trait Summary {
    fn summarize(&self) -> String;
}

// 带默认实现
trait Summary {
    fn summarize(&self) -> String {
        "abc"
    }
}

// 具体结构体实现
struct ABC {}
impl Summary for ABC {
    fn summarize(&self) -> String {
        "cde"
    }
}

// trait 作为参数
fn func_1(item: &impl Summary) {

}
// trait bound写法
fn func_1<T: Summary>(item: &T) {

}

// 多个trait参数，且每个参数实际的结构体可以不相同！
fn func_2(item1: &impl Summary, item2: &impl Summary) {

}
// 多个trait参数，且每个参数实际的结构体必须相同！这个只有trait bound写法能实现
fn func_2<T: Summary>(item1: &T, item2: &T) {

}

// 指定多个trait 注意括号位置
fn func_3(item: &(impl Summary + Display)) {

}
// 指定多个trait bound写法
fn func_3<T: Summary + Display>(item: &T) {

}

// 使用trait bound + where 一般用于trait信息比较长
fn func_4<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Summary + Debug,
{

}

// 返回值为trait
fn func_5() -> impl Summary {}
// 返回值必须有确定的类型，不能有选择。例如如下这个是编译过不了的
fn func_fault(b: bool) -> impl Summary {
    if b {
        Summary1{}
    } else {
        Summary2{}
    }
}

// 返回值为trait 且入参的F函数满足返回类型为T
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

// 对满足Display trait的T 实现 ToString trait
// 这个被称作blanket implementations
// 相当于套娃，或者说选择性的继承，有点go语言interface的思想，灵活性极大，牛啊！
impl<T: Display> ToString for T {

}

let s = 3.to_string();
```