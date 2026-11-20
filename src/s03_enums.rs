//! Step 03: 泛型枚举 —— 亲手造一遍 Option 和 Result
//!
//! 你每天都在用的两个类型，定义其实只有这么几行:
//!
//! ```text
//!     enum Option<T>    { Some(T), None }
//!     enum Result<T, E> { Ok(T),   Err(E) }
//! ```
//!
//! 它们没有任何魔法，就是普通的泛型枚举。`.map()`、`.and_then()`、`.unwrap_or()`
//! 也都只是 `impl<T> Option<T>` 里的普通泛型方法。
//! 这一步我们把它们重新实现一遍（换个名字，免得和标准库的混淆）:
//!
//! ```text
//!     Maybe<T>       = Just(T) | Nothing          对应 Option<T>
//!     Outcome<T, E>  = Good(T) | Bad(E)           对应 Result<T, E>
//! ```
//!
//! 重点体会"类型在变换中怎么流动"。以 map 为例:
//!
//! ```text
//!     Maybe<T>  ──map(f: T -> U)──>  Maybe<U>
//!
//!     Just(x)   ──>  Just(f(x))        盒子里的值换了，类型也可以跟着换（T 变成 U）
//!     Nothing   ──>  Nothing           没有值，f 根本不会被调用
//! ```
//!
//! 这里 T 是"盒子原来装的类型"（属于 impl 块），U 是"变换之后的类型"（属于 map 这个方法，
//! 每次调用时由闭包的返回类型决定）。
//!
//! 测试: cargo test --test test_03_enums

// TODO 1: 定义两个枚举（测试要比较和打印它们，所以都加上 #[derive(Debug, Clone, PartialEq)]）:
//
//     pub enum Maybe<T>      { Just(T), Nothing }
//     pub enum Outcome<T, E> { Good(T), Bad(E) }
//
//   并给 Maybe<T> 实现:
//     pub fn is_just(&self) -> bool
//     pub fn unwrap_or(self, default: T) -> T       Just(x) -> x，Nothing -> default
//
//   会用到的语法:
//     match self { Maybe::Just(x) => ..., Maybe::Nothing => ... }
//     matches!(self, Maybe::Just(_))                 模式匹配得到 bool 的简写
#[derive(Debug, Clone, PartialEq)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome<T, E> {
    Good(T),
    Bad(E),
}

impl<T> Maybe<T> {
    pub fn is_just(&self) -> bool {
        match self {
            Maybe::Just(x) => true,
            Maybe::Nothing => false,
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Maybe::Just(x) => x,
            Maybe::Nothing => default,
        }
    }
    // TODO 2: 在同一个 impl<T> Maybe<T> 块里继续实现两个"变换"方法（签名需要你自己写）:
    //
    //     map:       Maybe<T> + (T -> U)         =>  Maybe<U>
    //     and_then:  Maybe<T> + (T -> Maybe<U>)  =>  Maybe<U>
    //
    //   两者的区别：map 的闭包返回一个"裸"的 U，由 map 负责重新装进 Just；
    //   and_then 的闭包自己返回 Maybe<U>（它可以决定"这一步失败了"，返回 Nothing），
    //   所以 and_then 不能再包一层，否则会得到 Maybe<Maybe<U>>。
    //
    //   会用到的语法:
    //     pub fn map<U, F>(self, f: F) -> Maybe<U>
    //     where
    //         F: FnOnce(T) -> U,
    pub fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U
    {
        match self {
            Maybe::Just(x) => Maybe::Just(f(x)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    pub fn and_then<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> Maybe<U>
    {
        match self {
            Maybe::Just(x) => f(x),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    // TODO 3a: 还是在这个 impl 块里:
    //
    //     pub fn ok_or<E>(self, err: E) -> Outcome<T, E>
    //                                   Just(x) -> Good(x)，Nothing -> Bad(err)
    //
    //   注意 E 是这个方法自己引入的类型参数：Maybe<T> 本身对"错误类型"一无所知，
    //   是调用方在这一刻决定错误用什么类型来表示。
    pub fn ok_or<E>(self, err: E) -> Outcome<T, E> {
        match self {
            Maybe::Just(x) => Outcome::Good(x),
            Maybe::Nothing => Outcome::Bad(err),
        }
    }
}

// TODO 3b: 给 Outcome<T, E> 写一个 impl 块（这次 impl 后面要声明两个类型参数），实现:
//
//     map:      只变换 Good 里的值:  Outcome<T, E> + (T -> U)  =>  Outcome<U, E>
//     map_err:  只变换 Bad 里的值:   Outcome<T, E> + (E -> G)  =>  Outcome<T, G>
//     ok:       丢掉错误信息:        Outcome<T, E>             =>  Maybe<T>
//
//   观察：map 的签名里 E 原封不动地从输入"穿"到了输出；map_err 则是 T 原封不动。
//         光看签名就能知道一个泛型函数"不可能"动哪些东西，这是泛型带来的额外保证。
impl<T,E> Outcome<T,E> {
    pub fn map<U, F>(self, f: F) -> Outcome<U,E>
    where F: FnOnce(T) -> U{
        match self {
            Outcome::Good(x) => Outcome::Good(f(x)),
            Outcome::Bad(err) => Outcome::Bad(err)
        }
    }
}

// TODO 4: 和标准库的 Option 互相转换。实现两个 From:
//
//     impl<T> From<Option<T>> for Maybe<T>      Some(x) -> Just(x)，None -> Nothing
//     impl<T> From<Maybe<T>> for Option<T>      反过来
//
//   这是你第一次"为泛型类型实现一个泛型 trait"，注意每个 <T> 的角色:
//     impl<T>                  声明 T
//     From<Option<T>>          trait 的类型参数：从什么类型转换过来
//     for Maybe<T>             给谁实现
//
//   实现了 From 之后，.into() 是白送的（标准库里有一条
//   `impl<T, U: From<T>> Into<U> for T`，这种"一次性给一大批类型实现 trait"的写法 Step 07 细讲）。

// TODO 5: 只给"嵌套的 Maybe"实现一个方法:
//
//     impl<T> Maybe<Maybe<T>>
//         pub fn flatten(self) -> Maybe<T>      Just(Just(x)) -> Just(x)，其余 -> Nothing
//
//   这和 Step 02 的 `impl Stack<f64>` 是同一个思路：impl 后面的类型不一定是"最一般"的形状，
//   可以是任何你想要的具体化程度。这里只有 Maybe<Maybe<某个 T>> 才有 flatten，
//   Maybe<i32> 上调用 flatten 会编译失败。

// ---------------------------------------------------------------
// 思考题
// ---------------------------------------------------------------
// 1. `let x = Maybe::Nothing;` 单独这一行为什么编译不过？
//    （和 `let v = Vec::new();` 之后什么都不 push 是同一个问题。）
//
// 2. Maybe<Maybe<T>>::flatten 能不能用 and_then 一行写出来？
//
// 3. 标准库里 `Option<T>::map` 的 F 用的是 FnOnce 而不是 Fn，为什么 FnOnce 是"最宽松"的选择？
