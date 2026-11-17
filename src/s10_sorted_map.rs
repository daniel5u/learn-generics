//! Step 10（综合）: 从零实现一个泛型容器 SortedMap<K, V>
//!
//! 前九步学的东西，标准库的每个容器都在同时使用。这一步我们自己造一个，
//! 把它"接入"Rust 的生态：能 collect() 出来、能用在 for 循环里、能用 map["key"] 取值、能 {:?} 打印。
//!
//! 数据结构本身很简单 —— 一个按 key 排好序的 Vec:
//!
//! ```text
//!     SortedMap<K, V> { entries: Vec<(K, V)> }        始终保持: key 严格递增，没有重复
//!
//!     查找:  二分查找 O(log n)
//!     插入:  二分找到位置，再 Vec::insert，O(n)
//! ```
//!
//! 重点不在算法，而在"每个 impl 块上应该写什么 bound"。请坚持 Step 02 的原则:
//! bound 写在需要它的地方，struct 定义上一个 bound 都不要加。
//!
//! ```text
//!     要做的事                         需要的能力
//!     ------------------------------   --------------------------------
//!     new / len / is_empty / iter      什么都不需要
//!     insert / get / remove            K: Ord         （要比较 key 才能二分）
//!     用 &str 去查 String 的 key        K: Borrow<Q>   （本步的新知识，见 TODO 2）
//!     collect() 成 SortedMap           实现 FromIterator
//!     for (k, v) in map                实现 IntoIterator
//!     map["key"]                       实现 Index
//!     {:?}                             K: Debug, V: Debug
//! ```
//!
//! 测试: cargo test --test test_10_sorted_map

use std::borrow::Borrow;
use std::fmt::{self, Debug};
use std::ops::Index;

// TODO 1: 定义结构体和基本操作。
//
//     pub struct SortedMap<K, V> { entries: Vec<(K, V)> }
//
//     impl<K, V> SortedMap<K, V>                       <- 没有 bound
//         pub fn new() -> Self
//         pub fn len(&self) -> usize
//         pub fn is_empty(&self) -> bool
//
//     impl<K: Ord, V> SortedMap<K, V>
//         pub fn insert(&mut self, key: K, value: V) -> Option<V>
//             key 不存在: 插到正确的位置（保持有序），返回 None
//             key 已存在: 用新 value 替换，返回 Some(旧 value)
//
//     再手写一个 impl<K, V> Default（不要 derive，原因见 Step 02 TODO 4）。
//
//   会用到的 API:
//     self.entries.binary_search_by(|(k, _)| k.cmp(&key))
//         返回 Result<usize, usize>:
//           Ok(i)   找到了，在下标 i
//           Err(i)  没找到，但如果要插入并保持有序，应该插在下标 i
//     vec.insert(i, item)
//     std::mem::replace(&mut slot, new)      把 new 放进去，把旧值拿出来

    // TODO 4a（做到 TODO 4 时再回来，写在无 bound 的 impl<K, V> 块里）:
    //
    //     pub fn iter(&self) -> impl Iterator<Item = (&K, &V)>       按 key 从小到大
    //     pub fn keys(&self) -> impl Iterator<Item = &K>
    //     pub fn map_values<U, F>(self, f: F) -> SortedMap<K, U>
    //     where
    //         F: FnMut(V) -> U,                key 不变，每个 value 交给 f 变换（V 可以变成另一种类型 U）
    //
    //   会用到的 API:
    //     self.entries.iter().map(|(k, v)| (k, v))       &(K, V) -> (&K, &V)

    // TODO 2: 查找 —— 以及为什么需要 Borrow（写在 impl<K: Ord, V> 块里）。
    //
    //   最直接的签名是 `pub fn get(&self, key: &K) -> Option<&V>`。
    //   但对 SortedMap<String, i32> 来说，这意味着查找时必须传 &String:
    //
    //       map.get(&"apple".to_string())        为了查一下，先堆分配一个 String，太浪费
    //       map.get("apple")                     我们想要的写法，但 "apple" 是 &str，不是 &String
    //
    //   标准库的解法是 Borrow trait。`K: Borrow<Q>` 的意思是：
    //   "可以从 &K 借出一个 &Q，并且 Q 的比较 / 哈希结果和 K 完全一致"。
    //   String: Borrow<str>、Vec<T>: Borrow<[T]>，以及任何 T: Borrow<T>（所以传 &K 依然可以）。
    //
    //       pub fn get<Q>(&self, key: &Q) -> Option<&V>
    //       where
    //           K: Borrow<Q>,
    //           Q: Ord + ?Sized,
    //
    //   逐条读:
    //     Q            调用方传进来的"查询用的类型"，每次调用可以不同
    //     K: Borrow<Q> 我存的 key 能借成 Q 来看
    //     Q: Ord       比较是在 Q 上做的
    //     ?Sized       Q 经常是 str 或 [T] 这种大小未知的类型（Step 06 TODO 5）
    //
    //   同样的签名风格再实现两个:
    //       pub fn contains_key<Q>(&self, key: &Q) -> bool
    //       pub fn remove<Q>(&mut self, key: &Q) -> Option<V>          删除并返回 value；不存在返回 None
    //
    //   会用到的 API:
    //     k.borrow()                    &K -> &Q
    //     self.entries.binary_search_by(|(k, _)| k.borrow().cmp(key))
    //     vec.remove(i)                 删除下标 i 的元素并返回它

// TODO 3: 让 SortedMap 能被 collect() 出来、能被 extend。
//
//     impl<K: Ord, V> FromIterator<(K, V)> for SortedMap<K, V>
//         fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
//
//     impl<K: Ord, V> Extend<(K, V)> for SortedMap<K, V>
//         fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I)
//
//   `iter.collect::<SortedMap<_, _>>()` 之所以能工作，是因为 collect 的签名是
//       fn collect<B: FromIterator<Self::Item>>(self) -> B
//   —— 一个只出现在返回值里的类型参数 B（和 Step 01 的 parse_all 一样），由调用方指定。
//   你给 SortedMap 实现了 FromIterator<(K, V)>，它就成了 B 的合法选择之一。
//
//   注意 FromIterator<(K, V)> 是泛型 trait（类型参数是"元素类型"）：
//   同一个容器可以从不同类型的元素构建，比如 String 同时实现了
//   FromIterator<char>、FromIterator<&str>、FromIterator<String>。
//
//   重复的 key：后出现的覆盖先出现的（直接复用 insert 就是这个行为）。

// TODO 4b: 让 SortedMap 能用在 for 循环里（先回到上面完成 TODO 4a）。
//
//   for 循环是 IntoIterator 的语法糖。它有两个关联类型需要你填:
//
//     impl<K, V> IntoIterator for SortedMap<K, V> {             for (k, v) in map      —— 消耗 map
//         type Item = (K, V);
//         type IntoIter = std::vec::IntoIter<(K, V)>;           直接复用 Vec 的迭代器类型
//         fn into_iter(self) -> Self::IntoIter
//     }
//
//     impl<'a, K, V> IntoIterator for &'a SortedMap<K, V> {     for (k, v) in &map     —— 只借用
//         type Item = &'a (K, V);
//         type IntoIter = std::slice::Iter<'a, (K, V)>;
//         fn into_iter(self) -> Self::IntoIter
//     }
//
//   第二个 impl 是给"引用类型" &'a SortedMap<K, V> 实现 trait —— impl 的对象可以是任何类型，包括引用。
//   这里必须手写 'a：关联类型 Item 里要提到"借用持续多久"，没有地方可以省略。
//
//   为什么关联类型里不能写 impl Iterator？因为关联类型必须是一个写得出名字的具体类型
//   （稳定版 Rust 的限制），所以这里借用 Vec / 切片现成的迭代器类型。

// TODO 5: 最后两个 trait。
//
//   (a) 下标取值 map[key]；key 不存在时 panic（和 HashMap 的行为一致）:
//
//         impl<K, V, Q> Index<&Q> for SortedMap<K, V>
//         where
//             K: Ord + Borrow<Q>,
//             Q: Ord + ?Sized,
//         {
//             type Output = V;
//             fn index(&self, key: &Q) -> &V
//         }
//
//       Index<Idx> 又是"类型参数 + 关联类型"的组合（Step 05）：
//       Idx 是输入（可以用不同类型的东西当下标），Output 是输出（取出来的类型是确定的）。
//       map["apple"] 中 Idx = &str，也就是 &Q 且 Q = str。
//
//       会用到的 API:
//         self.get(key).expect("key not found in SortedMap")
//
//   (b) Debug 打印，格式和标准库的 map 一样: {"a": 1, "b": 2}
//
//         impl<K: Debug, V: Debug> Debug for SortedMap<K, V>
//
//       会用到的 API:
//         f.debug_map().entries(self.iter()).finish()
//                                       需要先完成 TODO 4a 的 iter()
//
//       这次 bound 是 Debug 而不是 Ord：打印不需要比较，比较不需要打印。每个 impl 只要它真正需要的。

// ---------------------------------------------------------------
// 做完之后
// ---------------------------------------------------------------
// 1. 打开标准库 BTreeMap 的文档（https://doc.rust-lang.org/std/collections/struct.BTreeMap.html），
//    翻到 "Trait Implementations" 一节。现在你应该能读懂每一个 impl 的签名，
//    以及它上面的每一个 bound 为什么在那里。
//
// 2. 还可以继续加的东西（都没有测试，自己发挥）:
//      - impl<K: Ord, V, const N: usize> From<[(K, V); N]> for SortedMap<K, V>      （Step 08 的 const 泛型）
//      - impl<K: PartialEq, V: PartialEq> PartialEq for SortedMap<K, V>
//      - pub fn range<Q>(&self, from: &Q, to: &Q) -> impl Iterator<Item = (&K, &V)>
//      - 一个 entry(key).or_insert(value) 风格的 API
//
// 3. 回到 src/lib.rs，删掉 #![allow(...)] 那一行，看看有没有遗留的警告。
