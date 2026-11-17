//! Step 02: 泛型结构体与 impl 块
//!
//! 结构体也可以带类型参数：
//!
//! ```text
//!     struct Stack<T> { items: Vec<T> }
//! ```
//!
//! `Stack` 本身不是一个类型，它是一个"类型的模板"；`Stack<i32>`、`Stack<String>` 才是类型，
//! 而且它们是两个互不相干的类型。
//!
//! 给泛型结构体写方法时，最容易困惑的是 impl 后面的那个 `<T>`:
//!
//! ```text
//!     impl<T> Stack<T> { ... }
//!         ^         ^
//!         |         └ 使用：我要给 Stack<T> 写方法
//!         └ 声明：T 是一个类型参数（"对于任意类型 T"）
//! ```
//!
//! 为什么要先声明？因为 impl 后面的尖括号里也可以写具体类型，编译器得分清这两种情况:
//!
//! ```text
//!     impl<T> Stack<T>              对所有 T 都有的方法
//!     impl<T: Display> Stack<T>     只有当 T 能打印时才有的方法（"条件方法"）
//!     impl Stack<f64>               只有 Stack<f64> 才有的方法（这里没有类型参数了）
//! ```
//!
//! 三个 impl 块可以同时存在。这是 Rust 泛型很有表现力的地方：
//! 一个类型有哪些方法，取决于它的类型参数满足哪些条件。
//!
//! 约定俗成：bound 尽量写在"需要它的 impl / 方法"上，而不是写在 struct 定义上。
//! `struct Stack<T: Display>` 会让每一个提到 Stack<T> 的地方都被迫重复这个 bound。
//!
//! 测试: cargo test --test test_02_structs

use std::fmt::Display;

// TODO 1: 定义 `pub struct Stack<T>`（内部用一个 Vec<T> 存数据，栈顶 = Vec 的末尾），
//         并在 `impl<T> Stack<T>` 里实现下面这些方法:
//
//     pub fn new() -> Self
//     pub fn push(&mut self, item: T)
//     pub fn pop(&mut self) -> Option<T>         弹出栈顶；空栈返回 None
//     pub fn peek(&self) -> Option<&T>           看一眼栈顶，不弹出
//     pub fn len(&self) -> usize
//     pub fn is_empty(&self) -> bool
//
//   会用到的 API:
//     Vec::new()  vec.push(x)  vec.pop()  vec.last()  vec.len()  vec.is_empty()
//
//   注意这一组方法没有任何 bound：一个栈不需要了解它装的是什么。
//   在 impl 块里，`Self` 就是 `Stack<T>` 的简写。

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
// TODO 2: 条件方法 —— 再写两个 impl 块，各带一个 bound:
//
//     impl<T: Display> Stack<T>
//         pub fn render(&self) -> String        从栈底到栈顶: "[1, 2, 3]"，空栈: "[]"
//
//     impl<T: PartialOrd> Stack<T>
//         pub fn max(&self) -> Option<&T>       栈里最大的元素；空栈返回 None
//
//   会用到的 API:
//     x.to_string()                 Display 的类型都有这个方法（为什么？Step 07 揭晓）
//     strings.join(", ")            Vec<String> -> String
//
//   试一试：做完之后，在测试里对一个 Stack<Vec<i32>> 调用 .render()，读一下报错。
//           Vec<i32> 没有实现 Display，所以这个方法对它"不存在"——但 push / pop 照常可用。

impl<T: Display> Stack<T> {
    pub fn render(&self) -> String {
        let ans: Vec<String> = self.items.iter().map(|x| x.to_string()).collect();

        format!("[{}]", ans.join(", "))
    }
}
impl<T: PartialOrd> Stack<T> {
    pub fn max(&self) -> Option<&T> {
        let mut ans = self.items.first()?;

        for item in &self.items[1..] {
            if item > ans {
                ans = item
            }
        }

        Some(ans)
    }
}

// TODO 3: 只属于某一个具体类型的方法:
//
//     impl Stack<f64>
//         pub fn sum(&self) -> f64              空栈返回 0.0
//
//   注意 impl 后面没有 <T>：这里没有"任意类型"，只有 f64。
impl Stack<f64> {
    pub fn sum(&self) -> f64 {
        let mut ans: f64 = 0.0;

        for item in &self.items {
            ans += item;
        }

        ans
    }
}

// TODO 4: 手动为 Stack<T> 实现 Default（default() 返回空栈），要求对"任意 T"都成立。
//
//     impl<T> Default for Stack<T> { fn default() -> Self { ... } }
//
//   为什么不直接 #[derive(Default)]？
//     derive 生成的代码是 `impl<T: Default> Default for Stack<T>`——它会机械地给每个类型参数
//     都加上同名的 bound，哪怕根本用不上（空的 Vec<T> 并不需要一个"默认的 T"）。
//     于是 Stack<某个没有 Default 的类型> 就莫名其妙地不能 default() 了。
//   试一试：先用 derive，跑一下测试 t4_，看看报错；再换成手写的 impl。
//   （derive(Clone)、derive(Debug)、derive(PartialEq) 都有同样的行为，Step 08 还会再碰到它。）

/// 两个字段可以是不同类型的"一对"
impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pair<A, B> {
    pub first: A,
    pub second: B,
}

// TODO 5: 给 Pair<A, B> 写方法。重点：方法自己也可以再引入新的类型参数。
//
//     impl<A, B> Pair<A, B>
//         pub fn new(first: A, second: B) -> Self
//         pub fn swap(self) -> Pair<B, A>
//         pub fn mixup<C, D>(self, other: Pair<C, D>) -> Pair<A, D>
//                                               取 self 的 first 和 other 的 second
//         pub fn map_first<C, F>(self, f: F) -> Pair<C, B>
//         where
//             F: FnOnce(A) -> C,                把 first 交给 f 处理，second 原样保留
//
//   两层类型参数的区别:
//     A、B 属于 impl 块      —— 在"拿到一个 Pair"的那一刻就已经确定了
//     C、D、F 属于方法       —— 每次调用这个方法时才确定，同一个 Pair 类型可以用不同的 C 去调
//
//   `F: FnOnce(A) -> C` 读作："F 是某种可以被调用一次的东西，吃一个 A，吐一个 C"。
//   闭包为什么必须用泛型来接收，Step 09 会细讲，这里先照着用。

impl<A, B> Pair<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self {
            first: first,
            second: second,
        }
    }
    pub fn swap(self) -> Pair<B, A> {
        Pair {
            first: self.second,
            second: self.first
        }
    }
    pub fn mixup<C, D>(self, other: Pair<C, D>) -> Pair<A, D> {
        Pair {
            first: self.first,
            second: other.second
        }
    }
    pub fn map_first<C, F>(self, f: F) -> Pair<C, B>
    where F: FnOnce(A) -> C
    {
        Pair {
            first: f(self.first),
            second: self.second
        }
    }
}

// ---------------------------------------------------------------
// 思考题
// ---------------------------------------------------------------
// 1. `impl Stack<T> { ... }`（漏掉 impl 后面的 <T>）会报什么错？为什么编译器不能"猜"出 T 是泛型？
//
// 2. 如果同时写了
//        impl<T> Stack<T>   { pub fn describe(&self) -> &str { "generic" } }
//        impl    Stack<f64> { pub fn describe(&self) -> &str { "floats"  } }
//    会发生什么？（提示：Rust 不允许固有方法重名重叠，它没有 C++ 那种"模板特化"。）
