## cargo
- cargo new [PackageName]: 创建工程
- cargo new --lib [PackageName]: 创建rust库
- cargo build: 构建/编译
- cargo build --release: 编译速度较慢 可执行文件性能提升10倍 backtrace不显示行号
- cargo run: 运行
- cargo run --release: 编译速度较慢 可执行文件性能提升10倍 backtrace不显示行号
- cargo clippy: lint check
- cargo fmt: 格式化 (安装rustfmt之后可用)
- cargo tree: 查询依赖关系
- cargo bench: benchmark
- cargo udeps: 查看未使用到的依赖
- cargo doc: 生成文档

# overflow
```
整型溢出
比方说有一个 u8 ，它可以存放从零到 255 的值。那么当你将其修改为 256 时会发生什么呢？
这被称为 “整型溢出”（“integer overflow” ），这会导致以下两种行为之一的发生。
当在 debug 模式编译时，Rust 检查这类问题并使程序 panic，这个术语被 Rust 用来表明程序因错误而退出。

使用 --release flag 在 release 模式中构建时，Rust 不会检测会导致 panic 的整型溢出。
相反发生整型溢出时，Rust 会进行一种被称为二进制补码 wrapping（two’s complement wrapping）的操作。
简而言之，比此类型能容纳最大值还大的值会回绕到最小值，值 256 变成 0，值 257 变成 1，依此类推。
程序不会 panic，不过变量可能也不会是你所期望的值。依赖整型溢出 wrapping 的行为被认为是一种错误。

为了显式地处理溢出的可能性，可以使用这几类标准库提供的原始数字类型方法：

所有模式下都可以使用 wrapping_* 方法进行 wrapping，如 wrapping_add
如果 checked_* 方法出现溢出，则返回 None值
用 overflowing_* 方法返回值和一个布尔值，表示是否出现溢出
用 saturating_* 方法在值的最小值或最大值处进行饱和处理
```

# slice
```
字符串 slice，正如你想象的那样，是针对字符串的。不过也有更通用的 slice 类型。考虑一下这个数组：

let a = [1, 2, 3, 4, 5];
就跟我们想要获取字符串的一部分那样，我们也会想要引用数组的一部分。我们可以这样做：

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
这个 slice 的类型是 &[i32]。它跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。你可以对其他所有集合使用这类 slice。
```

## generics
泛型类型参数是否会有运行时消耗?  
不会。Rust通过在编译时进行泛型代码的单态化(monomorphization)来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。  
其实就是转化为了静态模版，在编译时解决了这个问题。