//! Step 04: Trait bound 的各种写法
//!
//! 泛型和 trait 是一对：泛型说"这里可以是任意类型"，trait bound 说"但它必须会做这些事"。
//! 这一步把 bound 能出现的位置和写法全部过一遍。
//!
//! ```text
//!     写法                                         含义
//!     ------------------------------------------   ---------------------------------------
//!     fn f<T: Shape>(x: &T)                        最基本的写法
//!     fn f(x: &impl Shape)                         上面的简写（"匿名的类型参数"）
//!     fn f<T>(x: &T) where T: Shape                where 子句，bound 多或者长的时候用
//!     fn f() -> impl Shape                         返回"某一个实现了 Shape 的类型"，但不告诉你是哪个
//!     impl<A: Shape, B: Shape> Shape for (A, B)    条件实现：只有 A、B 都是 Shape 时才成立
//! ```
//!
//! 两个容易搞混的点:
//!
//! 1. 参数位置的 `impl Trait` —— 每写一次就是一个"独立的"类型参数:
//!
//! ```text
//!     fn f(a: &impl Shape, b: &impl Shape)      a、b 可以是两种不同的类型（等价于 <T: Shape, U: Shape>）
//!     fn f<T: Shape>(a: &T, b: &T)              a、b 必须是同一种类型
//! ```
//!
//! 2. 参数位置和返回位置的 `impl Trait` 方向相反:
//!
//! ```text
//!     参数位置:   调用方决定具体类型，函数体必须对任何类型都成立
//!     返回位置:   函数体决定具体类型（只能是一种！），调用方只知道"它实现了这个 trait"
//! ```
//!
//! 测试: cargo test --test test_04_traits

use std::collections::HashMap;
use std::fmt::Display;

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    /// 默认方法：trait 自带实现，实现者可以不写。格式: "circle (area 3.14)"，面积保留两位小数
    fn describe(&self) -> String {
        // TODO 1a: 用 self.name() 和 self.area() 拼出描述。
        //
        //   会用到的 API:
        //     format!("{} (area {:.2})", ..., ...)
        //
        //   默认方法其实也是泛型代码：写它的时候你不知道 Self 是什么类型，
        //   只知道"Self 实现了 Shape"，所以只能调用 Shape 里声明过的方法。
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub r: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub w: f64,
    pub h: f64,
}

// TODO 1b: 为 Circle 和 Rect 实现 Shape。
//
//     Circle:  area = π r²，name = "circle"
//     Rect:    area = w * h，name = "rect"
//
//   会用到的 API:
//     std::f64::consts::PI

// TODO 2: 三个函数，三种写 bound 的方式（签名都要你自己写）:
//
//   (a) total_area: 一个切片里所有形状的面积之和。用 `<T: Shape>` 的写法。
//           total_area(&[Rect, Rect, ...]) -> f64
//
//   (b) bigger_name: 返回面积较大的那个形状的 name()（一样大时返回 a 的）。
//       a 和 b 可以是不同类型，用参数位置的 `impl Shape`。
//           bigger_name(&circle, &rect) -> String
//
//   (c) larger: 返回面积较大的那个形状本身（一样大时返回 a）。
//       因为要"返回其中一个"，a 和 b 必须是同一种类型，所以这里必须用具名的 T。
//           larger(rect1, rect2) -> Rect
//
//   思考：(c) 能不能写成 `fn larger(a: impl Shape, b: impl Shape) -> impl Shape`？
//         试一下，读一下报错，对照文件开头的第 2 点。

// TODO 3: where 子句。实现:
//
//     pub fn summarize<K, V>(shapes: &HashMap<K, V>) -> Vec<String>
//
//   把 map 里的每一项变成 "key: describe()" 这样的一行，并且按 key 从小到大排序。
//   例如 {"b" => Rect 2x3, "a" => Circle r=1}  ->  ["a: circle (area 3.14)", "b: rect (area 6.00)"]
//
//   K 和 V 各自需要什么能力？自己分析，然后写进 where 子句:
//     - key 要被打印进字符串      -> ?
//     - key 要能排序              -> ?（"全序"的 trait，比 PartialOrd 更强的那个）
//     - value 要能 describe()     -> ?
//
//   会用到的 API:
//     map.iter()                    产生 (&K, &V)
//     vec.sort_by(|a, b| a.0.cmp(b.0))     或者 sort_by_key；先收集成 Vec<(&K, &V)> 再排序
//
//   注意：我们没有要求 K: Hash + Eq。只是"遍历"一个 HashMap 并不需要哈希，
//         标准库把 bound 精确地放在了需要它的方法（insert / get）上，而不是 struct 上——
//         这就是 Step 02 说的那条约定。

// TODO 4: 返回位置的 impl Trait。实现两个函数:
//
//     pub fn make_square(side: f64) -> impl Shape
//         返回一个边长为 side 的正方形（内部用 Rect 就行）。
//         调用方拿到的东西"只能当 Shape 用"：它访问不了 .w / .h，因为签名没有承诺返回的是 Rect。
//         好处：以后你把内部实现换成一个新的 Square 类型，所有调用方都不用改。
//
//     pub fn scaled_areas(shapes: &[Rect], k: f64) -> impl Iterator<Item = f64>
//         返回一个迭代器，依次产生每个矩形的"面积 × k"。
//         这是返回位置 impl Trait 最常见的用途：迭代器适配器（map / filter……）叠出来的类型
//         又长又包含闭包（闭包的类型根本写不出来），只能说"它是某个 Iterator"。
//
//   会用到的 API:
//     shapes.iter().map(move |s| ...)     move 把 k 拷进闭包里，否则闭包借用的是即将消失的局部变量 k
//
//   试一试：让 make_square 在 side < 0 时返回 Circle、否则返回 Rect，看看报错。
//           返回位置的 impl Trait 只能对应"一种"具体类型；想返回不同类型需要 Step 06 的 dyn。

// TODO 5: 条件实现 —— 让"两个形状组成的元组"也是一个形状:
//
//     impl<A: Shape, B: Shape> Shape for (A, B)
//         area = 两者面积之和
//         name = "{A 的 name}+{B 的 name}"，例如 "circle+rect"
//
//   做完之后，下面这些会自动成立（想想为什么，编译器是怎么一层层推出来的）:
//     (Circle, Rect)                    是 Shape
//     ((Circle, Rect), Rect)            也是 Shape —— A = (Circle, Rect)，它是 Shape 是因为上一行
//     total_area(&[(c1, r1), (c2, r2)]) 能用   —— T = (Circle, Rect)

// ---------------------------------------------------------------
// 思考题
// ---------------------------------------------------------------
// 1. total_area(&[circle, rect]) 为什么不行？（提示：切片 &[T] 里的 T 只能是一种类型。Step 06 解决它。）
//
// 2. `fn f(x: impl Shape)` 和 `fn f<T: Shape>(x: T)` 几乎等价，唯一的区别是什么？
//    （提示：调用方能不能写 f::<Circle>(...)？）
