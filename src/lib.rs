//! learn-generics: 通过 10 个 Step 搞懂 Rust 的泛型
//!
//! 每个 Step 是 src/ 下的一个文件，对应 tests/ 下的一个测试文件。
//! 路线图和用法见 README.md。

// 练习骨架里有很多 todo!() 和暂时没用到的参数/导入，先把这些警告关掉，
// 免得真正有用的编译错误被淹没。全部做完之后可以删掉这一行，看看还剩什么警告。
#![allow(dead_code, unused_variables, unused_imports)]

pub mod s01_functions;
pub mod s02_structs;
pub mod s03_enums;
pub mod s04_traits;
pub mod s05_assoc;
pub mod s06_dispatch;
pub mod s07_blanket;
pub mod s08_const_phantom;
pub mod s09_closures_lifetimes;
pub mod s10_sorted_map;
