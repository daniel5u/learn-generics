//! Step 07: Blanket impl、扩展 trait 与孤儿规则
//!
//! 到现在为止，impl 的对象都是"某一个类型"或"某一族类型"（Stack<T>、(A, B)）。
//! 其实 `for` 后面可以直接写一个光秃秃的类型参数:
//!
//! ```text
//!     impl<T: Display> Shout for T { ... }
//!                                ^ 给"所有"实现了 Display 的类型，一次性实现 Shout
//! ```
//!
//! 这叫 blanket impl（"一揽子实现"）。标准库里到处都是:
//!
//! ```text
//!     impl<T: Display> ToString for T          所以任何能 {} 打印的类型都有 .to_string()
//!     impl<T, U: From<T>> Into<U> for T        所以实现了 From 就白送 .into()
//!     impl<I: Iterator> IntoIterator for I     所以迭代器自己可以直接用在 for 循环里
//! ```
//!
//! 它的一个常见用途是"扩展 trait"（extension trait）：给别人的类型加方法。
//! 比如想给所有迭代器加一个 .min_max() —— 定义一个新 trait，再 blanket impl 给所有 Iterator。
//! itertools 这个库就是这么做的。
//!
//! 但 impl 不是想写就能写，有两条规则管着:
//!
//! ```text
//!     孤儿规则（orphan rule）:  impl Trait for Type 时，Trait 和 Type 至少有一个是"你自己的 crate 定义的"。
//!                              -> 你不能给 Vec<T> 实现 Display：两个都是标准库的。
//!                              -> 绕过办法：newtype，包一层自己的类型 `struct Wrapper<T>(Vec<T>)`。
//!
//!     不重叠（coherence）:      同一个 trait，任何类型最多只能匹配上一个 impl。
//!                              -> 写了 impl<T: Display> Shout for T 之后，
//!                                 再写 impl Shout for i32 就会报 "conflicting implementations"。
//!                                 Rust（稳定版）没有"特化"：不能"通用版 + 某类型的特殊版"。
//! ```
//!
//! 两条规则的目的相同：保证"某类型的某 trait 用哪个实现"在整个程序里只有唯一答案，
//! 不会因为多引入了一个依赖库，同一行代码的行为就变了。
//!
//! 测试: cargo test --test test_07_blanket

use std::fmt::{self, Display};
use std::ops::Deref;

pub trait Shout {
    /// 大写 + 感叹号: 42 -> "42!"，"hi" -> "HI!"
    fn shout(&self) -> String;
}

// TODO 1: 第一个 blanket impl。
//
//     impl<T: Display> Shout for T
//
//   会用到的 API:
//     self.to_string()              —— 它本身就来自标准库的 blanket impl
//     s.to_uppercase()
//
//   做完之后 i32、f64、&str、String、char、bool…… 以及"将来任何人写的任何实现了 Display 的类型"
//   全都自动有了 .shout()。

// TODO 2: 扩展 trait —— 给所有迭代器加两个方法。
//
//   定义:
//     pub trait IterExt: Iterator {
//         fn tally_if<P>(self, pred: P) -> usize
//         where
//             Self: Sized,
//             P: FnMut(&Self::Item) -> bool,
//         { ... }                                  有多少个元素满足 pred
//
//         fn min_max(self) -> Option<(Self::Item, Self::Item)>
//         where
//             Self: Sized,
//             Self::Item: PartialOrd + Clone,
//         { ... }                                  一次遍历同时求出 (最小值, 最大值)；空迭代器返回 None
//     }
//
//   然后一行 blanket impl，把它发给所有迭代器（方法都有默认实现，所以大括号里是空的）:
//     impl<I: Iterator> IterExt for I {}
//
//   几个新语法:
//     trait IterExt: Iterator       "supertrait"：实现 IterExt 的类型必须也实现了 Iterator，
//                                   所以默认方法里可以用 self.next()、Self::Item、for x in self
//     where Self: Sized             按值接收 self 的默认方法需要知道 Self 的大小；
//                                   同时它让 dyn 可以跳过这个方法（Step 06 思考题 2）
//     Self::Item: PartialOrd + Clone   bound 可以只加在某一个方法上：
//                                   元素不可比较的迭代器照样能用 tally_if，只是没有 min_max
//
//   min_max 为什么要 Clone？只有一个元素时，它既是最小值也是最大值，要返回两份。

// TODO 3: 孤儿规则与 newtype。
//
//   我们想让 Vec<T> 能用 {} 打印成 "[1, 2, 3]"，但
//       impl<T: Display> Display for Vec<T>
//   违反孤儿规则（Display 和 Vec 都不是我们的）。先把这行写出来看一眼报错（E0117），再删掉。
//
//   正确做法：定义一个自己的包装类型（newtype），给它实现:
//
//     pub struct Wrapper<T>(pub Vec<T>);
//
//     impl<T: Display> Display for Wrapper<T>
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
//         输出 "[1, 2, 3]"；空的输出 "[]"
//
//   包了一层之后，Vec 的方法（len、iter、下标……）都用不了了。再实现 Deref 把它们"借"回来:
//
//     impl<T> Deref for Wrapper<T> { type Target = Vec<T>; fn deref(&self) -> &Vec<T> }
//
//   会用到的 API:
//     write!(f, "[")?;   write!(f, "{item}")?;   最后返回 write!(f, "]") 或 Ok(())
//     self.0                        元组结构体的第一个字段

pub trait Describe {
    fn describe(&self) -> String;
}

// TODO 4: 不用 blanket impl，而是"一组互不重叠的 impl" —— 编译器会像递归一样把它们拼起来。
//
//     impl Describe for i32                              数字本身: "7"
//     impl Describe for bool                             "yes" / "no"
//     impl<T: Describe> Describe for Option<T>           "some(<里面的 describe>)" / "none"
//     impl<T: Describe> Describe for Vec<T>              "[a, b, c]"，每个元素用它自己的 describe
//
//   于是 vec![Some(true), None].describe() 的求解过程是:
//       Vec<Option<bool>>: Describe ?
//         └ 需要 Option<bool>: Describe ?
//             └ 需要 bool: Describe ?   有！
//   这一切发生在编译期，最后生成的是一份为 Vec<Option<bool>> 定制的代码。
//   （serde 的 Serialize、标准库的 Debug / Clone / PartialEq 都是这样层层组合出来的。）

// ---------------------------------------------------------------
// 思考题（建议都动手试一下，读报错）
// ---------------------------------------------------------------
// 1. 在 TODO 1 之后再加一行 `impl Shout for i32 { ... }`，报什么错（E0119）？
//
// 2. 在 TODO 4 里再加一个 `impl<T: Display> Describe for T`，为什么和 `impl Describe for Vec<T>` 冲突？
//    Vec<T> 明明没有实现 Display 啊？
//    （提示：标准库"将来"可以给 Vec 加上 Display；孤儿规则要保证那一天你的代码不会突然坏掉。）
//
// 3. 为什么 `impl<T> From<T> for Wrapper<T>` 不行，而 `impl<T> From<Vec<T>> for Wrapper<T>` 可以？
//    （提示：标准库已有 `impl<T> From<T> for T`。当 T = Wrapper<X> 时……）
