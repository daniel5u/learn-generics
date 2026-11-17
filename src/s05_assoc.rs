//! Step 05: 泛型 trait vs 关联类型
//!
//! trait 自己也可以带类型，而且有两种带法:
//!
//! ```text
//!     trait From<T>   { fn from(value: T) -> Self; }                 类型参数（写在尖括号里）
//!     trait Iterator  { type Item; fn next(&mut self) -> Option<Self::Item>; }   关联类型（写在 trait 里面）
//! ```
//!
//! 区别只有一个，但非常关键 —— "同一个类型能实现这个 trait 几次":
//!
//! ```text
//!     类型参数:  From<T> 对每个不同的 T 都是一个不同的 trait，
//!                所以 String 可以同时实现 From<&str>、From<char>、From<Box<str>>……
//!                => T 是"输入"：由使用者来选
//!
//!     关联类型:  一个类型只能实现 Iterator 一次，Item 在那一次实现里被定死。
//!                => Item 是"输出"：由实现者决定，知道了 Self 就唯一确定了 Item
//! ```
//!
//! 假如 Iterator 被设计成 `trait Iterator<T>`，那么一个类型可以同时是 Iterator<i32> 和
//! Iterator<String>，每次调用 .next() 编译器都得问你"你要哪一个？"——所以它用了关联类型。
//!
//! 很多 trait 两种都有。比如加法:
//!
//! ```text
//!     trait Add<Rhs = Self> { type Output; fn add(self, rhs: Rhs) -> Self::Output; }
//!
//!     Rhs    是类型参数:  同一个类型可以"加"不同类型的东西（Vec2 + Vec2，Vec2 + f64……）
//!     Output 是关联类型:  一旦确定了"谁 + 谁"，结果的类型就是唯一的
//!     = Self 是默认值:    只写 Add 就等于 Add<Self>
//! ```
//!
//! 用到关联类型的 bound 写法:
//!
//! ```text
//!     I: Iterator<Item = u32>                  要求 Item 恰好是 u32
//!     I: Iterator, I::Item: Display            不限定 Item 是什么，但要求它能打印
//!     T: Add<Output = T>                       T + T 的结果还是 T
//! ```
//!
//! 测试: cargo test --test test_05_assoc

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

// TODO 1: 泛型 trait —— 给同一个类型实现"多次" From。
//
//     impl From<Celsius> for Fahrenheit         F = C × 9/5 + 32
//     impl From<Fahrenheit> for Celsius         C = (F − 32) × 5/9
//     impl From<f64> for Fahrenheit             把裸数字直接当作华氏度
//
//   这样 Fahrenheit 就同时实现了 From<Celsius> 和 From<f64>：两个不同的 trait，互不冲突。
//
//   然后写一个泛型函数，接受"任何能转成 Fahrenheit 的东西"，返回华氏度的数值:
//
//     pub fn fahrenheit_value<T: Into<Fahrenheit>>(t: T) -> f64
//
//   `T: Into<Fahrenheit>` 是 API 设计里很常见的手法：调用方可以传 Celsius、Fahrenheit、f64，
//   函数内部第一行统一 .into() 一下。（Fahrenheit 自己为什么也行？因为标准库有 `impl<T> From<T> for T`。）

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

// TODO 2: 运算符重载 —— 类型参数和关联类型同时出场。
//
//   (a) 向量 + 向量:
//         impl<T: Add<Output = T>> Add for Vec2<T> { type Output = Vec2<T>; ... }
//       bound 的意思：T 自己得能相加，而且加出来还是 T（这样结果才能放回 Vec2<T>）。
//
//   (b) 向量 × 标量（每个分量都乘以同一个数）:
//         impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> { type Output = Vec2<T>; ... }
//       这里 trait 的类型参数 Rhs 不再是默认的 Self，而是 T: "Vec2<T> 乘以一个 T"。
//       为什么需要 Copy？因为 rhs 这一个值要被用两次（乘 x 一次，乘 y 一次）。
//
//   这也回答了 Step 01 的思考题 1：`fn double<T: Add<Output = T> + Copy>(x: T) -> T { x + x }`。

/// 一个带关联类型的 trait："按下标取元素的容器"
pub trait Container {
    /// 容器里装的元素类型，由每个实现者自己指定
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn size(&self) -> usize;

    /// 默认方法里同样可以用 Self::Item
    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }
}

/// "同一个值重复 times 次"的虚拟容器，只存了一份 value
pub struct Repeat<T> {
    pub value: T,
    pub times: usize,
}

// TODO 3: 实现并使用 Container。
//
//   (a) impl<T> Container for Vec<T>        Item = T
//       impl<T> Container for Repeat<T>     Item = T；index < times 时返回 Some(&self.value)
//
//   (b) 两个使用它的泛型函数:
//
//         pub fn last_item<C: Container>(c: &C) -> Option<&C::Item>
//             返回最后一个元素；空容器返回 None
//
//         pub fn contains<C>(c: &C, target: &C::Item) -> bool
//         where
//             C: Container,
//             C::Item: PartialEq,
//             容器里有没有等于 target 的元素
//
//   注意 (b) 的签名里只有一个类型参数 C，元素类型是"顺着 C 查出来的"(C::Item)。
//   如果 Container 被设计成 Container<T>，这两个函数就都得写成 <C: Container<T>, T>，多拖一个参数。

/// 倒计时: Countdown(3) 依次产生 3, 2, 1，然后结束
pub struct Countdown(pub u32);

// TODO 4: 实现 Iterator —— 标准库里最重要的带关联类型的 trait。
//
//   (a) impl Iterator for Countdown         type Item = u32
//       只需要实现 next()；map / filter / sum / collect 等几十个方法全是 trait 的默认方法，白送。
//
//   (b) 一个泛型的迭代器适配器 Pairs：把任意迭代器的元素两两打包。
//         [1, 2, 3, 4, 5]  ->  (1, 2), (3, 4)        落单的最后一个丢掉
//
//         pub struct Pairs<I> { inner: I }
//
//         impl<I: Iterator> Iterator for Pairs<I> {
//             type Item = (I::Item, I::Item);         <- 我的 Item 由"里面那个迭代器的 Item"算出来
//             ...
//         }
//
//       再加一个方便构造的函数:
//         pub fn pairs<I: IntoIterator>(iter: I) -> Pairs<I::IntoIter>
//
//       IntoIterator 是"能变成迭代器的东西"（Vec、数组、范围、迭代器自己……），
//       它有两个关联类型：Item 和 IntoIter（变出来的那个迭代器的类型）。for 循环用的就是它。
//
//   会用到的 API:
//     self.inner.next()?            在返回 Option 的函数里，? 遇到 None 会直接返回 None
//     iter.into_iter()

// TODO 5: 把这一步学的全用上 —— 一个对"任何可相加的元素"求和的函数（签名自己写）:
//
//     sum_all(vec![1, 2, 3])                 -> 6
//     sum_all([0.5, 0.25])                   -> 0.75
//     sum_all(Countdown(4))                  -> 10
//     sum_all(vec![Vec2{..}, Vec2{..}])      -> Vec2{..}     需要先给 Vec2 加上 #[derive(Default)]
//     sum_all(Vec::<i32>::new())             -> 0            空的时候返回"零"
//
//   分析需要哪些 bound:
//     - 参数是"能变成迭代器的东西"              I: IntoIterator
//     - 返回值的类型是它的元素类型              -> I::Item
//     - 元素之间能相加，且结果还是元素类型       I::Item: Add<Output = I::Item>
//     - 需要一个"零"作为起点                    I::Item: Default      （数字的 default 是 0）
//
//   会用到的 API:
//     T::default()                  这里的 T 要写成 I::Item::default()
//     iter.fold(init, |acc, x| acc + x)

// ---------------------------------------------------------------
// 思考题
// ---------------------------------------------------------------
// 1. 标准库的 `trait Deref { type Target: ?Sized; ... }` 为什么用关联类型而不是 Deref<Target>？
//    如果 String 能同时 Deref 到 str 和 [u8]，`*s` 该是什么类型？
//
// 2. 能不能同时写 `impl Mul<f64> for Vec2<f64>` 和 `impl<T: ...> Mul<T> for Vec2<T>`？为什么？
//
// 3. `fn f(it: impl Iterator<Item = u32>)` 和 `fn f<I: Iterator>(it: I) where I::Item: Into<u64>`
//    各自能接受 Countdown 吗？能接受 `vec![1u8, 2].into_iter()` 吗？
