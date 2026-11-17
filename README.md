# learn-generics

通过 10 个 Step 搞懂 Rust 的泛型（generics）：怎么写、编译器怎么处理它、标准库怎么用它。

## 怎么用

每个 Step 是 `src/` 下的一个文件，配 `tests/` 下的一个测试文件：

1. 打开 `src/sXX_*.rs`，读文件开头的讲解。
2. 按顺序完成里面的 `TODO`（每个 TODO 下面都有"会用到的语法 / API"提示）。
3. 跑这一步的测试，直到全绿：

```bash
cargo test --test test_01_functions          # 跑 Step 01 的全部测试
cargo test --test test_01_functions t2_      # 只跑 TODO 2 的测试（测试名以 t2_ 开头）
cargo test                                   # 全部 Step
```

### 和 Python 版教程不一样的地方

在 Rust 里，**"测试编译不过"就是第一道测试**。泛型的大部分知识点（缺 bound、类型对不上、
生命周期不够长）都体现为编译错误，而不是运行时的断言失败。所以每个 Step 的节奏是：

```text
编译错误（缺函数 / 签名不够泛型 / 缺 bound）
   └─> 读报错，改签名  ──>  编译通过，但 todo!() panic
                                └─> 写函数体  ──>  测试通过
```

几个实用技巧：

- **一个测试文件要整体编译通过才能运行。** 如果想先验证 TODO 1，可以先把后面几个 TODO 的
  签名写出来、函数体留 `todo!()`，让文件先编译过，再用 `t1_` 过滤着跑。
- **认真读 `rustc` 的报错。** 它通常会直接告诉你缺哪个 bound（`help: consider restricting type parameter T`）。
  报错里的错误码可以查详细解释：`rustc --explain E0277`。
- 文件里标了"试一试"的地方，是故意让你写一段**编译不过**的代码去看报错。看完删掉就行。
- 每个文件末尾有"思考题"，不用写代码，但建议想清楚再进下一步。

## 路线图

| Step | 文件 | 主题 | 学完能回答 |
| ---- | ---- | ---- | ---------- |
| 01 | `s01_functions.rs` | 泛型函数、trait bound、turbofish | 为什么只写 `<T>` 时什么都干不了？ |
| 02 | `s02_structs.rs` | 泛型结构体、`impl<T>`、条件方法 | `impl<T> Stack<T>` 里为什么有两个 `<T>`？ |
| 03 | `s03_enums.rs` | 泛型枚举：手写 `Option` / `Result` | `map` 的签名里 `T`、`U`、`F` 各是谁决定的？ |
| 04 | `s04_traits.rs` | bound 的各种写法、`impl Trait`、`where` | 参数位置和返回位置的 `impl Trait` 有什么不同？ |
| 05 | `s05_assoc.rs` | 泛型 trait vs 关联类型、运算符、`Iterator` | `From<T>` 为什么用类型参数，而 `Iterator` 用 `type Item`？ |
| 06 | `s06_dispatch.rs` | 单态化 vs `dyn`、胖指针、`?Sized` | 泛型编译之后变成了什么？什么时候该用 `dyn`？ |
| 07 | `s07_blanket.rs` | blanket impl、扩展 trait、孤儿规则、newtype | 为什么实现了 `Display` 就自动有 `.to_string()`？ |
| 08 | `s08_const_phantom.rs` | const 泛型、`PhantomData`、typestate | 怎么让"矩阵尺寸不匹配""单位混用"变成编译错误？ |
| 09 | `s09_closures_lifetimes.rs` | `Fn` / `FnMut` / `FnOnce`、生命周期参数 | 为什么接收闭包必须用泛型？`'a` 和 `T` 像在哪里？ |
| 10 | `s10_sorted_map.rs` | 综合：实现 `SortedMap<K, V>` | 标准库容器上那一大串 impl 和 bound 都是干什么的？ |

Step 01–05 是"怎么写"，06 是"它是怎么工作的"，07–09 是进阶用法，10 把所有东西用在一个真实的容器上。
各 Step 之间的代码互不依赖，但讲解是按顺序递进的，建议从头做。

## 一页速查

```rust
fn f<T>(x: T)                           // 类型参数
fn f<T: Debug + Clone>(x: T)            // trait bound
fn f<T>(x: T) where T: Debug            // where 子句（等价）
fn f(x: impl Debug)                     // 参数位置 impl Trait = 匿名类型参数
fn f() -> impl Debug                    // 返回位置 impl Trait = 隐藏的某一个具体类型
fn f(x: &dyn Debug)                     // trait object：运行时分发，不是泛型
fn f<T: ?Sized>(x: &T)                  // 放宽默认的 Sized 要求
fn f<const N: usize>(x: [u8; N])        // const 泛型
fn f<'a>(x: &'a str) -> &'a str         // 生命周期参数
fn f<F: Fn(i32) -> i32>(g: F)           // 闭包 bound

struct S<T> { .. }                      // 泛型结构体
impl<T> S<T> { .. }                     // 对所有 T
impl<T: Display> S<T> { .. }            // 只对满足 bound 的 T
impl S<f64> { .. }                      // 只对一个具体类型
impl<T: Display> MyTrait for T { .. }   // blanket impl

trait Tr<T> { .. }                      // 泛型 trait：一个类型可以实现多次
trait Tr { type Out; .. }               // 关联类型：一个类型只能实现一次
f::<i32>(..)   "5".parse::<i32>()       // turbofish：手动指定类型参数
```
