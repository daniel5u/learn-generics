//! Step 08: const 泛型与 PhantomData —— 把更多信息塞进类型里
//!
//! 泛型参数一共有三种。前七步用的都是第一种:
//!
//! ```text
//!     <T>                 类型参数
//!     <const N: usize>    const 参数：一个编译期就确定的"值"         （本步前半）
//!     <'a>                生命周期参数                              （Step 09）
//! ```
//!
//! ## const 泛型
//!
//! 数组的长度是类型的一部分：[i32; 3] 和 [i32; 4] 是不同的类型。
//! const 泛型让我们可以对"长度"抽象:
//!
//! ```text
//!     fn sum<const N: usize>(xs: [i32; N]) -> i32         对任意长度的数组都能用
//!     struct Matrix<const R: usize, const C: usize>       行数、列数写进类型
//! ```
//!
//! 一旦尺寸进了类型，"尺寸对不上"就从运行时 panic 变成了编译错误:
//!
//! ```text
//!     Matrix<2, 3> × Matrix<3, 4>  ->  Matrix<2, 4>       OK
//!     Matrix<2, 3> × Matrix<2, 3>                         编译不过：3 ≠ 2
//! ```
//!
//! ## PhantomData
//!
//! 有时我们想要一个类型参数"只起标记作用"，结构体里并不真的存它:
//!
//! ```text
//!     struct Length<Unit> { value: f64 }                  编译错误 E0392: parameter `Unit` is never used
//!     struct Length<Unit> { value: f64, _unit: PhantomData<Unit> }      OK
//! ```
//!
//! PhantomData<T> 是一个大小为 0 的类型，它的作用就是告诉编译器"假装我这里有一个 T"。
//! 运行时它完全不存在：Length<Meters> 在内存里就是一个 f64。
//! 但在编译期，Length<Meters> 和 Length<Feet> 是两个不同的类型，不能混用——
//! 1999 年 NASA 的火星气候探测器就是因为公制和英制单位混用而坠毁的，这种 bug 类型系统能直接拦住。
//!
//! 同样的手法还能表达"状态"（typestate 模式）：Door<Open> 和 Door<Closed> 是不同的类型，
//! 只有 Door<Open> 上才有 walk_through() 方法，"穿过一扇关着的门"根本写不出来。
//!
//! 测试: cargo test --test test_08_const_phantom

use std::marker::PhantomData;
use std::ops::Add;

// TODO 1: 第一个 const 泛型函数（签名自己写）。
//
//     first_n: 取切片的前 N 个元素，组成一个长度为 N 的数组；切片不够长时，后面用默认值补齐。
//
//         let a: [i32; 3] = first_n(&[1, 2, 3, 4, 5]);     // [1, 2, 3]       N 从返回类型推断
//         first_n::<u8, 4>(&[7])                           // [7, 0, 0, 0]    turbofish 同时指定 T 和 N
//
//   会用到的语法:
//     pub fn first_n<T, const N: usize>(items: &[T]) -> [T; N]       T 上还需要两个 bound，自己分析
//     [T::default(); N]             "N 个相同的值"组成的数组，要求这个值是 Copy 的
//     函数体里 N 可以当成一个普通的 usize 常量来用

// TODO 2: 尺寸写在类型里的矩阵。
//
//     #[derive(Debug, Clone, Copy, PartialEq)]
//     pub struct Matrix<const R: usize, const C: usize> {
//         pub data: [[f64; C]; R],          R 行，每行 C 个数
//     }
//
//     impl<const R: usize, const C: usize> Matrix<R, C>
//         pub fn new(data: [[f64; C]; R]) -> Self
//         pub fn zeros() -> Self
//         pub fn shape(&self) -> (usize, usize)            (R, C)
//         pub fn transpose(&self) -> Matrix<C, R>          转置：行列互换，注意返回类型里 R、C 换了位置
//         pub fn mul<const K: usize>(&self, other: &Matrix<C, K>) -> Matrix<R, K>
//                                                          矩阵乘法: out[i][j] = Σ_k self[i][k] * other[k][j]
//
//   看 mul 的签名：self 是 R×C，other 必须是 C×K（同一个 C！），结果是 R×K。
//   "左边的列数 = 右边的行数" 这条规则，现在由类型检查器替你把关。
//   K 是 mul 这个方法自己的 const 参数，和 Step 02 里方法自带的 <C, D> 是一回事。
//
//   会用到的 API:
//     [[0.0; C]; R]                 全 0 的 R×C 二维数组
//     Matrix::<C, R>::zeros()       先造一个全 0 的结果，再用下标循环填进去

/// 单位标记。它们没有字段、不会被创建出实例，只在类型层面用来区分。
/// 注意：这两个类型故意什么 trait 都没有 derive。
pub struct Meters;
pub struct Feet;

// TODO 3: 带单位的长度。
//
//     pub struct Length<U> { value: f64, _unit: PhantomData<U> }
//
//     impl<U> Length<U>
//         pub fn new(value: f64) -> Self
//         pub fn value(&self) -> f64
//
//     impl<U> Add for Length<U>                 同单位才能相加，结果还是这个单位
//
//     impl Length<Feet>   { pub fn to_meters(self) -> Length<Meters> }      1 英尺 = 0.3048 米
//     impl Length<Meters> { pub fn to_feet(self) -> Length<Feet> }
//
//   还有一个要求：Length<U> 必须是 Copy 的（它本质上就是一个 f64），而且对"任意 U"都成立。
//   这里会再次撞上 Step 02 TODO 4 讲的 derive 问题:
//     #[derive(Clone, Copy)] 生成的是 impl<U: Copy> Copy for Length<U>，
//     而 Meters / Feet 并不是 Copy，于是 Length<Meters> 就不是 Copy。
//   所以要手写:
//     impl<U> Clone for Length<U> { fn clone(&self) -> Self { *self } }
//     impl<U> Copy for Length<U> {}
//
//   会用到的 API:
//     PhantomData                   它既是类型也是值：字段初始化写 `_unit: PhantomData` 即可

/// 门的状态。NAME 是"关联常量"：和关联类型类似，只不过每个实现者提供的是一个常量值。
pub trait DoorState {
    const NAME: &'static str;
}

pub struct Open;
pub struct Closed;

// TODO 4: typestate —— 用类型表示状态，非法的状态转换直接编译不过。
//
//     impl DoorState for Open     NAME = "open"
//     impl DoorState for Closed   NAME = "closed"
//
//     pub struct Door<S> { ... }                    需要 PhantomData<S>；可以再存一个 opened: u32 计数
//
//     impl Door<Closed>
//         pub fn new() -> Self                      新门是关着的
//         pub fn open(self) -> Door<Open>           开门，计数 +1
//
//     impl Door<Open>
//         pub fn close(self) -> Door<Closed>
//         pub fn walk_through(&self) -> &'static str        返回 "walked through"
//
//     impl<S: DoorState> Door<S>                    任何状态下都有的方法
//         pub fn state(&self) -> &'static str       返回 S::NAME
//         pub fn times_opened(&self) -> u32
//
//   注意 open / close 接收的是 self（不是 &self）：旧状态的门被"消耗"掉了，
//   调用之后原来的变量不能再用，你手里只剩下新状态的门。这是所有权和泛型配合出来的效果。
//
//   state() 里的 S::NAME：我们没有 S 的任何实例（它只在 PhantomData 里），
//   但依然可以通过类型本身取到它的关联常量——信息完全来自类型。

// ---------------------------------------------------------------
// 试一试（每个都应该"编译失败"，读一下报错再删掉）
// ---------------------------------------------------------------
// 1. let a: Matrix<2, 3> = Matrix::zeros();  a.mul(&a);
// 2. Length::<Meters>::new(1.0) + Length::<Feet>::new(1.0)
// 3. Door::new().walk_through()
// 4. let d = Door::new();  let o = d.open();  d.state();        （d 已经被 move 走了）
//
// 思考题: std::mem::size_of::<Length<Meters>>() 是多少？size_of::<Door<Open>>() 呢？
