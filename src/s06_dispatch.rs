//! Step 06: 泛型是怎么"工作"的 —— 单态化（静态分发）vs dyn（动态分发）
//!
//! 到目前为止写的泛型，编译之后都"消失"了。编译器做的事情叫单态化（monomorphization）:
//!
//! ```text
//!     你写的:                              编译器实际生成的（示意）:
//!
//!     fn speak_all<T: Animal>(xs: &[T])    fn speak_all_Dog(xs: &[Dog]) { ... 直接调用 Dog::speak ... }
//!                                          fn speak_all_Cat(xs: &[Cat]) { ... 直接调用 Cat::speak ... }
//!     speak_all(&dogs);
//!     speak_all(&cats);                    每个用到的具体类型，各复制一份函数体
//! ```
//!
//! - 好处：每一份都知道确切的类型，方法调用是普通的直接调用，可以内联 —— 泛型是"零成本"的。
//! - 代价：用了多少种类型就生成多少份代码（编译慢、二进制大）；而且类型必须在编译期确定，
//!   所以 `Vec<T>` 里只能放同一种类型。
//!
//! 另一条路是 trait object：`dyn Animal`。它表示"某个实现了 Animal 的类型，具体是哪个运行时才知道":
//!
//! ```text
//!     &dyn Animal / Box<dyn Animal>  是一个"胖指针"，占两个指针的大小:
//!
//!         ┌──────────┐
//!         │ data ptr ├──────> 真正的数据（Dog 或 Cat……）
//!         ├──────────┤
//!         │ vtable   ├──────> 这个类型的方法表: [drop, size, align, name, speak]
//!         └──────────┘
//!
//!     调用 x.speak() 时：先去 vtable 里查到函数地址，再跳过去 —— 这就是动态分发。
//! ```
//!
//! - 好处：函数只有一份；不同类型可以放进同一个 Vec<Box<dyn Animal>>。
//! - 代价：每次调用多一次间接跳转、不能内联；通常还要一次堆分配（Box）。
//!
//! ```text
//!                      泛型 <T: Trait>             dyn Trait
//!     类型何时确定      编译期                      运行期
//!     生成的代码        每种类型一份                只有一份
//!     方法调用          直接调用，可内联            查 vtable，间接调用
//!     混装不同类型      不行                        可以
//! ```
//!
//! 经验法则：默认用泛型；需要"混装"或者想减少代码膨胀时用 dyn。
//!
//! 测试: cargo test --test test_06_dispatch

use std::any::type_name;
use std::mem::size_of;

pub trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
}

pub struct Cat {
    pub name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn speak(&self) -> String {
        format!("{}: Woof!", self.name)
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn speak(&self) -> String {
        format!("{}: Meow!", self.name)
    }
}

/// 返回 T 这个类型的名字（字符串）
// TODO 1: 亲眼看看单态化。
//
//   (a) 实现 type_name_of：返回参数的类型名。
//
//       会用到的 API:
//         std::any::type_name::<T>()     返回 &'static str，比如 "i32"、"alloc::string::String"
//
//       这个函数的函数体里完全没用到参数 x，它只用了"T 是什么"这个信息。
//       T 在运行时并不存在；能拿到名字，正是因为编译器给每个 T 各生成了一份函数，
//       每一份里 type_name::<T>() 都已经是一个写死的字符串常量。
//
//   (b) 实现 chorus_static：让切片里的每只动物叫一声，收集成 Vec<String>。
//       用泛型 `<T: Animal>` —— 静态分发。
pub fn type_name_of<T>(x: &T) -> &'static str {
    todo!()
}

pub fn chorus_static(animals: &[Dog]) -> Vec<String> {
    todo!()
}

// TODO 2: 动态分发。
//
//   (a) chorus_dyn: 和 chorus_static 做一样的事，但参数是 &[Box<dyn Animal>]。
//       这个函数没有任何类型参数 —— 它不是泛型函数，编译后只有一份。
//
//   (b) zoo: 返回一个"混装"的 Vec，依次是:
//           Dog "Rex", Cat "Tom", Dog "Fido"
//
//   会用到的语法:
//     Box::new(Dog { name: "Rex".to_string() })     类型是 Box<Dog>
//     在需要 Box<dyn Animal> 的地方，Box<Dog> 会自动转换过去（unsized coercion）。
//     只要返回类型写明了 Vec<Box<dyn Animal>>，vec![Box::new(dog), Box::new(cat)] 就能通过。
pub fn chorus_dyn(animals: &[Box<dyn Animal>]) -> Vec<String> {
    todo!()
}

pub fn zoo() -> Vec<Box<dyn Animal>> {
    todo!()
}

/// kind 是 "dog" 或 "cat" 时造出对应的动物（名字用传入的 name），其他情况返回 None
// TODO 3: 运行时才决定返回哪种类型。
//
//   Step 04 里试过：`-> impl Animal` 只能返回一种具体类型，"if 返回 Dog else 返回 Cat" 编译不过。
//   因为 impl Trait 仍然是静态分发，编译器要在编译期知道返回值到底多大、调用哪个 speak。
//   而这里返回哪种动物取决于运行时的字符串，所以必须用 Box<dyn Animal>。
pub fn make_animal(kind: &str, name: &str) -> Option<Box<dyn Animal>> {
    todo!()
}

/// 返回三个大小（单位：字节）:
///     (&Dog 的大小, &dyn Animal 的大小, Box<dyn Animal> 的大小)
// TODO 4: 亲眼看看胖指针。
//
//   会用到的 API:
//     std::mem::size_of::<T>()       T 类型的值占多少字节
//
//   先预测一下三个数分别是多少（64 位机器上一个指针是 8 字节），再跑测试验证。
pub fn pointer_sizes() -> (usize, usize, usize) {
    todo!()
}

/// 让动物叫两声，用 " " 连接: "Rex: Woof! Rex: Woof!"
// TODO 5: ?Sized —— 让一个泛型函数同时接受具体类型和 dyn。
//
//   现在这个函数只接受 &Dog。把它改成泛型，使得下面两种调用都能编译:
//
//       speak_twice(&dog)            T = Dog
//       speak_twice(&*boxed)         T = dyn Animal      （boxed: Box<dyn Animal>）
//
//   如果只写 `<T: Animal>`，第二种会报错 "the size for values of type `dyn Animal` cannot be known"。
//   原因：每个类型参数都有一个"隐形"的默认 bound —— `T: Sized`（编译期知道大小）。
//   dyn Animal 的大小在编译期是未知的（可能是 Dog，也可能是 Cat），所以它不满足 Sized。
//
//   会用到的语法:
//     <T: Animal + ?Sized>          ?Sized 读作"不要求 Sized"，是唯一一个"放宽"而不是"收紧"的 bound
//
//   只要函数里始终通过引用（&T）来使用 T，就不需要知道它的大小，所以可以放宽。
pub fn speak_twice(animal: &Dog) -> String {
    todo!()
}

// ---------------------------------------------------------------
// 思考题
// ---------------------------------------------------------------
// 1. 不是所有 trait 都能写成 dyn。下面这个 trait 写 `Box<dyn Duplicate>` 会报错
//    "the trait `Duplicate` is not dyn compatible"，为什么？
//
//        trait Duplicate { fn dup(&self) -> Self; }
//
//    （提示：vtable 里的每个函数都要有确定的签名。通过 dyn 调用 dup 时，返回值的类型/大小是什么？
//      同理，带泛型参数的方法 `fn feed<F: Food>(&self, f: F)` 也不行：vtable 里放不下无限多份。）
//
// 2. Clone 就是上面这种形状，所以 `Box<dyn Clone>` 不存在。那 Iterator 有那么多带泛型的方法
//    （map、filter……），为什么 `Box<dyn Iterator<Item = u32>>` 却可以？
//    （提示：去看标准库里 map 的签名，末尾有一句 `where Self: Sized`。）
//
// 3. chorus_static(&zoo()) 能编译吗？要让它能编译，需要补一个什么 impl？
//    （提示：`impl<T: Animal + ?Sized> Animal for Box<T>`。写出来试试。）
