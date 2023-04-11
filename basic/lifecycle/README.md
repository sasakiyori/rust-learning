## 语法
```rust
// 生命周期注解是为了编译器能更好地检查代码错误的，例如检测悬垂引用(dangling references)。本身不会改变任何引用的生命周期长短
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用

// 在a'这个生命周期中 x、y还有返回值，都一定是有效的
// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
// 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 以下代码是错的！如果是其他语言则可能是对的，因为它没有所有权和borrow的概念
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        // string2的生命周期太短啦 编译器以它为准 发现错误
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// 结构体中的生命周期注解
// ImportantExcerpt实例不能活得比它的成员part的引用久！
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 生命周期省略规则(lifetime elision rules)
//第一条规则是编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数就有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
//第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
//第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法 (method)，那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。

// 静态生命周期
// 其生命周期能够存活于整个程序期间
let s: &'static str = "I have a static lifetime.";


// 地狱
// 结合生命周期和trait和泛型
// 写起来复杂，但是实际功能很简单啊。。
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```