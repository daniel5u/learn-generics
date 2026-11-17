//! Step 01: 泛型函数 —— 把"类型"也变成参数
//!
//! 普通函数的参数是"值"：`fn add_one(x: i32)`，调用时才知道 x 是几。
//! 泛型函数多了一类参数——"类型"：
//!
//! ```text
//!     fn identity<T>(x: T) -> T { x }
//!                 ^        ^
//!                 |        └ 使用：x 的类型是 T
//!                 └ 声明：这里有一个类型参数，名字叫 T
//! ```
//!
//! 调用时编译器会把 T "填"成具体类型：`identity(5)` 里 T = i32，`identity("hi")` 里 T = &str。
//!
//! 最重要的一条规则（和 C++ 模板、Python 鸭子类型都不一样）：
//!
//!     函数体只能对 T 做"签名里声明过"的事。
//!
//! 光写一个 `<T>`，编译器对 T 一无所知：不能比较、不能打印、不能相加、甚至不能复制，
//! 只能把它 move 来 move 去。想做更多，就要用 trait bound 把要求写进签名：
//!
//! ```text
//!     fn show<T: Debug>(x: T)              T 必须能用 {:?} 打印
//!     fn max<T: PartialOrd>(a: T, b: T)    T 必须能用 < > 比较
//!     fn f<T: Debug + Clone>(x: T)         多个要求用 + 连接
//!     fn f<T>(x: T) where T: Debug + Clone 要求太长时挪到 where 子句，意思完全一样
//! ```
//!
//! 这样做的好处：泛型函数在"定义处"就被完整检查了一遍。只要它自己能编译，
//! 那么对任何满足 bound 的类型都一定能用——报错永远出现在调用方"类型不满足 bound"这一行，
//! 而不是函数体深处某个莫名其妙的地方。
//!
//! 本 Step 的做法：下面的函数现在都只支持 i32（或干脆没写完）。
//! 你的任务是把它们改成泛型，让测试能"编译通过 + 运行通过"。
//! 注意：在 Rust 里"测试编译不过"本身就是一种测试失败，认真读编译器的报错，它通常直接告诉你缺哪个 bound。
//!
//! 测试: cargo test --test test_01_functions
//!       cargo test --test test_01_functions t2_      （只跑 TODO 2 的测试）

use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

/// (a, b) -> (b, a)，a 和 b 可以是两种不同的类型
// TODO 1: 改成泛型。需要两个类型参数（比如 A 和 B），不需要任何 bound。
//
//   思考：为什么这里一个 bound 都不用写？因为我们只是把值 move 到另一个位置，
//         没有对它做任何"需要了解它"的操作。
pub fn swap<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}

/// 返回切片里最大元素的引用；空切片返回 None
// TODO 2: 改成泛型，让它对 f64、&str、String……都能用。
//
//   会用到的语法:
//     fn largest<T: PartialOrd>(...)   PartialOrd 提供 < > <= >=
//
//   注意返回的是 Option<&T> 而不是 Option<T>：
//     返回 T 就得把元素从切片里"拿出来"，那需要 T: Copy 或 T: Clone，
//     String 这种类型就用不了（或者要付出克隆的代价）。返回引用则对任何 T 都成立。
//   试一试：先不写 `: PartialOrd`，看看编译器怎么说。
pub fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }

    let mut ans: &T = &items[0];

    for item in items {
        if item > ans {
            ans = item;
        }
    }
    Some(ans)
}

/// items 里有多少个元素等于 target
// TODO 3: 改成泛型。这次需要的能力是"判断相等"，对应的 trait 是 PartialEq（提供 == 和 !=）。
pub fn count_eq<T: PartialEq>(items: &[T], target: &T) -> usize {
    let mut ans: usize = 0;

    for item in items {
        if item == target {
            ans += 1;
        }
    }

    ans
}

/// 把每个字符串解析成 T，解析失败的直接跳过
// TODO 4: 改成泛型：返回 Vec<T>，T 是任何"能从字符串解析出来"的类型。
//
//   会用到的 API:
//     T: FromStr                 标准库里 "str -> Self" 的 trait，i32 / f64 / bool 等都实现了
//     s.parse::<T>()             返回 Result<T, T::Err>；::<T> 这种写法叫 turbofish
//     .ok()                      Result<T, E> -> Option<T>
//     iter.filter_map(...)       留下 Some 里的值，丢掉 None
//
//   这个函数的 T 只出现在返回值里，参数里没有，所以编译器没法从参数推断 T。
//   调用方必须告诉它 T 是什么，有两种写法（测试里都用到了）:
//     parse_all::<i32>(&["1", "2"])                 turbofish：直接指定类型参数
//     let v: Vec<f64> = parse_all(&["1.5"]);        标注接收变量的类型，让编译器反推
pub fn parse_all<T: FromStr>(inputs: &[&str]) -> Vec<T> {
    inputs.iter().filter_map(|x| x.parse::<T>().ok()).collect()
}

/// 用文字描述 a 和 b 的大小关系:
///     "1 < 2"    "2 > 1"    "1 == 1"    无法比较时: "NaN ? NaN"
/// 其中的值用 {:?} 格式打印（所以字符串会带引号: "\"a\" < \"b\""）
// TODO 5: 改成泛型。这次 T 需要同时满足两个要求：能比较、能用 {:?} 打印。
//         请用 where 子句来写 bound，练一下这种写法。
//
//   会用到的 API:
//     a.partial_cmp(b)           返回 Option<Ordering>
//                                —— PartialOrd 的 "Partial" 就体现在这：有些值之间没有大小关系
//                                   （比如 f64 的 NaN 和任何数比较都不成立），这时返回 None
//     Ordering::Less / Ordering::Equal / Ordering::Greater
//     format!("{a:?} < {b:?}")
pub fn describe_order<T>(a: &T, b: &T) -> String
where
    T: PartialOrd + Debug,
{
    return match a.partial_cmp(b) {
        Some(Ordering::Less) => format!("{a:?} < {b:?}"),
        Some(Ordering::Equal) => format!("{a:?} == {b:?}"),
        Some(Ordering::Greater) => format!("{a:?} > {b:?}"),
        None => format!("{a:?} ? {b:?}")
    }
}

// ---------------------------------------------------------------
// 思考题（不用写代码，想清楚了再去下一个 Step）
// ---------------------------------------------------------------
// 1. 下面的函数为什么编译不过？要加什么 bound 才行？（答案在 Step 05）
//
//        fn double<T>(x: T) -> T { x + x }
//
// 2. `largest::<i32>(&[])` 里为什么必须写 turbofish？去掉会报什么错？
//
// 3. `fn first<T>(items: &[T]) -> T { items[0] }` 为什么编译不过？
//    有哪两种修法（改返回类型 / 加 bound）？各自的代价是什么？
