//! Step 09: 闭包与生命周期 —— 它们其实也是泛型
//!
//! ## 闭包
//!
//! 每个闭包都有一个"独一无二、写不出名字"的类型。哪怕两个闭包的代码一模一样，类型也不同:
//!
//! ```text
//!     let f = |x: i32| x + 1;        类型: 编译器生成的匿名结构体 A（里面存着它捕获的变量）
//!     let g = |x: i32| x + 1;        类型: 另一个匿名结构体 B
//! ```
//!
//! 既然名字写不出来，函数想接收闭包就只有两条路，正好对应 Step 06 的两种分发:
//!
//! ```text
//!     fn apply<F: Fn(i32) -> i32>(f: F)      泛型：对每个闭包类型单态化一份，调用可以内联（最常用）
//!     fn apply(f: &dyn Fn(i32) -> i32)       trait object：只有一份代码，通过 vtable 调用
//! ```
//!
//! `Fn(i32) -> i32` 不是一个类型，而是一个 trait（带一点语法糖）。这样的 trait 有三个，
//! 区别在于"调用时闭包怎么使用它捕获的变量":
//!
//! ```text
//!     FnOnce    可能把捕获的值消耗掉          只能调用一次          所有闭包都实现
//!     FnMut     可能修改捕获的值              可以调用多次          不消耗捕获值的闭包实现
//!     Fn        只读取捕获的值                可以调用多次、可共享   不修改捕获值的闭包实现
//!
//!     Fn ⊂ FnMut ⊂ FnOnce      写 bound 时：要求越靠右越"宽松"，能接收的闭包越多
//! ```
//!
//! 所以写一个接收闭包的函数时，应该选"你实际需要的最宽松的那个":
//! 只调用一次就写 FnOnce（Option::map 就是）；要调用多次就写 FnMut；需要共享或并发调用才写 Fn。
//!
//! ## 生命周期
//!
//! 生命周期 `'a` 是第三种泛型参数。它和 `<T>` 的声明位置一样、"由调用方填入"的方式也一样，
//! 只是它抽象的不是类型，而是"一个引用在多大范围内有效":
//!
//! ```text
//!     fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
//!
//!     读作：存在某个范围 'a，a 和 b 在这个范围内都有效，返回的引用也只保证在这个范围内有效。
//!     调用时编译器会把 'a 填成"a、b 两者有效范围的交集"。
//! ```
//!
//! 生命周期不改变任何运行时行为，也不会让任何东西"活得更久"，它只是把
//! "返回的引用是从哪个参数里借来的"这件事写进签名，让编译器能在调用处检查。
//!
//! 大多数时候不用手写，因为有省略规则（elision）：只有一个引用参数时，返回值就默认借自它；
//! 有 &self 时默认借自 self。只有当编译器分不清"借自谁"时才需要你标出来。
//!
//! 测试: cargo test --test test_09_closures_lifetimes

use std::fmt::Display;

// TODO 1: 接收闭包（签名自己写，选对 Fn / FnMut / FnOnce）。
//
//   (a) apply_twice(f, x): 返回 f(f(x))
//           apply_twice(|x| x * 2, 5)  ->  20
//       f 被调用两次，且 f 的输入输出是同一个类型 T。
//
//   (b) call_n(n, f): 把 f 调用 n 次，f 没有参数也没有返回值。
//           let mut count = 0;
//           call_n(3, || count += 1);      // 闭包修改了它捕获的 count
//       这里 bound 应该选哪个，才能让上面的调用通过？
//       另外参数要写成 `mut f: F`：调用 FnMut 需要对闭包本身的可变访问。
//
//   会用到的语法:
//     F: Fn(T) -> T           F: FnMut()           括号里是参数类型，-> 后面是返回类型

// TODO 2: 返回闭包。闭包的类型写不出来，所以返回类型只能写 `impl Fn...`（Step 04 的返回位置 impl Trait）。
//
//   (a) make_adder(n): 返回一个"加 n"的闭包
//           pub fn make_adder(n: i32) -> impl Fn(i32) -> i32
//
//   (b) make_counter(): 返回一个计数器，每调用一次返回值加 1: 1, 2, 3, ...
//           pub fn make_counter() -> impl FnMut() -> u32
//       两次 make_counter() 得到的计数器互相独立，各自有各自的状态。
//
//   (c) compose(f, g): 返回一个新闭包，效果是"先 f 后 g"：x -> g(f(x))
//           pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C
//       三个类型参数描述了数据怎么流：A --f--> B --g--> C。B 是"中间类型"，必须对得上。
//
//   会用到的语法:
//     move |x| ...            把捕获的变量"移进"闭包里。返回闭包时几乎总是需要：
//                             不 move 的话闭包只是借用局部变量，而函数一返回局部变量就没了

// TODO 3: 把闭包存进结构体 —— 惰性求值。
//
//     pub struct Lazy<T, F> { ... }
//
//     impl<T, F: FnOnce() -> T> Lazy<T, F>
//         pub fn new(init: F) -> Self               只是把 init 存起来，不调用
//         pub fn get(&mut self) -> &T               第一次调用时才执行 init 并把结果缓存；之后直接返回缓存
//         pub fn is_initialized(&self) -> bool
//
//   结构体要存一个闭包，就必须有一个类型参数 F 来表示"那个写不出名字的闭包类型"。
//   所以 Lazy::new(|| 42) 的完整类型是 Lazy<i32, {某个闭包类型}>。
//
//   难点：init 是 FnOnce，调用它会把它消耗掉，而 get 拿到的只是 &mut self。
//   怎么从 &mut self 里把 F "拿出来"？
//
//   会用到的 API:
//     字段设计:  init: Option<F>,  value: Option<T>
//     self.init.take()              把 Option 里的值取走，原地留下 None —— 从 &mut 里"搬走"值的标准手法
//     self.value.as_ref()           &Option<T> -> Option<&T>
//     .unwrap() / .expect("...")

/// 在一段文本里找到的匹配：text 是 haystack 的一个子切片（不是拷贝）
#[derive(Debug, PartialEq)]
pub struct Highlight<'a> {
    pub text: &'a str,
    pub start: usize,
}

// TODO 4: 生命周期参数。
//
//   (a) longest(a, b): 返回较长的那个字符串切片（一样长时返回 a）。
//       两个引用参数，编译器不知道返回值借自哪个，需要你声明 <'a> 并标注。
//
//   (b) find(haystack, needle): 在 haystack 里找 needle 第一次出现的位置，
//       返回 Some(Highlight { text: haystack 中匹配的那一段, start: 起始字节下标 })，找不到返回 None。
//
//           pub fn find<'a>(haystack: &'a str, needle: &str) -> Option<Highlight<'a>>
//
//       关键：返回的 Highlight 只借用了 haystack，和 needle 无关。所以 'a 只标在 haystack 上，
//       needle 用它自己的（匿名的）生命周期。测试里会在 needle 已经销毁之后继续使用返回值——
//       如果你把 needle 也标成 &'a str，那个测试就编译不过。
//
//       结构体里存引用时，结构体本身就要带生命周期参数：Highlight<'a> 的意思是
//       "这个 Highlight 不能活得比它借用的那段文本更久"。
//
//   会用到的 API:
//     haystack.find(needle)         返回 Option<usize>（字节下标）
//     &haystack[start..start + needle.len()]

// TODO 5: 类型、闭包、生命周期三者同时出现。
//
//   (a) best_by(items, key): 返回 key 值最大的那个元素的引用；空切片返回 None；并列时返回最先出现的。
//           best_by(&["a", "ccc", "bb"], |s| s.len())   ->  Some(&"ccc")
//
//           pub fn best_by<T, K, F>(items: &[T], key: F) -> Option<&T>
//           where
//               K: PartialOrd,
//               F: Fn(&T) -> K,
//
//       三个类型参数各管一件事：T 是元素，K 是"用来比较的 key"，F 是算 key 的闭包。
//       返回的 &T 借自 items（省略规则自动处理了，不用手写 'a）。
//
//   (b) box_all(items): 把一组"能打印的东西"变成 trait object。
//           pub fn box_all<T: Display>(items: Vec<T>) -> Vec<Box<dyn Display>>
//
//       照这个签名写会报错: "the parameter type `T` may not live long enough"。
//       原因：Box<dyn Display> 其实是 Box<dyn Display + 'static> 的简写，意思是
//       "盒子里的东西不含任何会过期的借用"。而 T 可能是 &'短命 str 这样的类型。
//       修法是给 T 再加一个生命周期 bound:
//
//           T: Display + 'static
//
//       `T: 'static` 的意思不是"T 的值永远活着"，而是"T 这个类型里没有非 'static 的引用"。
//       String、i32、&'static str 都满足；借用了局部变量的 &str 不满足。

// ---------------------------------------------------------------
// 思考题
// ---------------------------------------------------------------
// 1. `F: Fn(&T) -> K` 里的 &T 没有写生命周期，它完整的写法是 `F: for<'x> Fn(&'x T) -> K`
//    （higher-ranked trait bound）：F 必须对"任意"生命周期的 &T 都能调用。
//    为什么这里不能用函数签名上的某个固定的 'a？（提示：best_by 内部借出去的引用，调用方看不见。）
//
// 2. fn 指针类型 `fn(i32) -> i32` 和 `impl Fn(i32) -> i32` 有什么区别？
//    `apply_twice(i32::abs, -3)` 为什么能编译？（普通函数也实现了 Fn 系列 trait。）
//
// 3. compose 返回的闭包捕获了 f 和 g。它的大小是多少？
//    试试 std::mem::size_of_val(&compose(|x: i32| x + 1, |x: i32| x * 2))，再想想为什么。
