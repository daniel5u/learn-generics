use learn_generics::s10_sorted_map::SortedMap;

fn fruit() -> SortedMap<String, u32> {
    let mut m = SortedMap::new();
    m.insert(String::from("pear"), 3);
    m.insert(String::from("apple"), 1);
    m.insert(String::from("mango"), 2);
    m
}

// ---------------------------------------------------------------
// TODO 1: 基本操作
// ---------------------------------------------------------------
#[test]
fn t1_new_is_empty() {
    let m: SortedMap<i32, i32> = SortedMap::new();
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
}

#[test]
fn t1_insert_new_keys() {
    let mut m = SortedMap::new();
    assert_eq!(m.insert(2, "two"), None);
    assert_eq!(m.insert(1, "one"), None);
    assert_eq!(m.insert(3, "three"), None);
    assert_eq!(m.len(), 3);
    assert!(!m.is_empty());
}

#[test]
fn t1_insert_existing_key_replaces_and_returns_old_value() {
    let mut m = SortedMap::new();
    assert_eq!(m.insert("k", 1), None);
    assert_eq!(m.insert("k", 2), Some(1));
    assert_eq!(m.insert("k", 3), Some(2));
    assert_eq!(m.len(), 1);
}

#[test]
fn t1_new_and_default_need_no_bounds() {
    // 这个类型不能比较、不能打印、没有默认值 —— 但空的 map 照样能建
    struct Opaque;
    let a: SortedMap<Opaque, Opaque> = SortedMap::new();
    let b: SortedMap<Opaque, Opaque> = SortedMap::default();
    assert_eq!(a.len() + b.len(), 0);
}

// ---------------------------------------------------------------
// TODO 2: 查找与 Borrow
// ---------------------------------------------------------------
#[test]
fn t2_get_with_the_key_type_itself() {
    let mut m = SortedMap::new();
    m.insert(10, 'a');
    m.insert(5, 'b');
    m.insert(20, 'c');
    assert_eq!(m.get(&5), Some(&'b'));
    assert_eq!(m.get(&10), Some(&'a'));
    assert_eq!(m.get(&20), Some(&'c'));
    assert_eq!(m.get(&7), None);
}

#[test]
fn t2_get_string_keys_with_str() {
    let m = fruit();
    // key 是 String，但可以直接用 &str 查，不需要分配一个 String
    assert_eq!(m.get("apple"), Some(&1));
    assert_eq!(m.get("pear"), Some(&3));
    assert_eq!(m.get("durian"), None);
    // 传 &String 也仍然可以
    assert_eq!(m.get(&String::from("mango")), Some(&2));
}

#[test]
fn t2_get_vec_keys_with_slice() {
    let mut m: SortedMap<Vec<u8>, &str> = SortedMap::new();
    m.insert(vec![1, 2, 3], "abc");
    let query: &[u8] = &[1, 2, 3];
    assert_eq!(m.get(query), Some(&"abc"));
}

#[test]
fn t2_contains_key_and_remove() {
    let mut m = fruit();
    assert!(m.contains_key("mango"));
    assert!(!m.contains_key("durian"));
    assert_eq!(m.remove("mango"), Some(2));
    assert_eq!(m.remove("mango"), None);
    assert!(!m.contains_key("mango"));
    assert_eq!(m.len(), 2);
    // 其余的 key 不受影响
    assert_eq!(m.get("apple"), Some(&1));
    assert_eq!(m.get("pear"), Some(&3));
}

#[test]
fn t2_many_inserts_stay_searchable() {
    let mut m = SortedMap::new();
    // 用一个固定的"伪随机"顺序插入 0..200
    for i in 0..200u32 {
        let k = (i * 73) % 200;
        m.insert(k, k * 2);
    }
    assert_eq!(m.len(), 200);
    for k in 0..200u32 {
        assert_eq!(m.get(&k), Some(&(k * 2)));
    }
}

// ---------------------------------------------------------------
// TODO 3: FromIterator / Extend
// ---------------------------------------------------------------
#[test]
fn t3_collect_into_sorted_map() {
    let m: SortedMap<&str, usize> = ["bb", "a", "ccc"].into_iter().map(|s| (s, s.len())).collect();
    assert_eq!(m.len(), 3);
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("ccc"), Some(&3));
}

#[test]
fn t3_collect_with_turbofish_and_duplicate_keys() {
    let m = vec![(1, "first"), (2, "x"), (1, "second")]
        .into_iter()
        .collect::<SortedMap<_, _>>();
    assert_eq!(m.len(), 2);
    assert_eq!(m.get(&1), Some(&"second"), "后出现的覆盖先出现的");
}

#[test]
fn t3_extend() {
    let mut m = fruit();
    m.extend(vec![(String::from("kiwi"), 9), (String::from("apple"), 100)]);
    assert_eq!(m.len(), 4);
    assert_eq!(m.get("kiwi"), Some(&9));
    assert_eq!(m.get("apple"), Some(&100));
}

// ---------------------------------------------------------------
// TODO 4: 迭代
// ---------------------------------------------------------------
#[test]
fn t4_iter_is_sorted_by_key() {
    let m = fruit();
    let got: Vec<(&String, &u32)> = m.iter().collect();
    let keys: Vec<&str> = got.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(keys, vec!["apple", "mango", "pear"]);
    let values: Vec<u32> = got.iter().map(|(_, v)| **v).collect();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn t4_keys() {
    let m: SortedMap<i32, ()> = [5, 1, 4, 1, 3].into_iter().map(|k| (k, ())).collect();
    let keys: Vec<i32> = m.keys().copied().collect();
    assert_eq!(keys, vec![1, 3, 4, 5]);
}

#[test]
fn t4_map_values_can_change_the_value_type() {
    let m: SortedMap<String, String> = fruit().map_values(|n| "*".repeat(n as usize));
    assert_eq!(m.get("apple").map(String::as_str), Some("*"));
    assert_eq!(m.get("pear").map(String::as_str), Some("***"));
    assert_eq!(m.len(), 3);
}

#[test]
fn t4_for_loop_by_reference() {
    let m = fruit();
    let mut total = 0;
    let mut names = Vec::new();
    for (name, count) in &m {
        names.push(name.as_str());
        total += count;
    }
    assert_eq!(names, vec!["apple", "mango", "pear"]);
    assert_eq!(total, 6);
    // m 只是被借用，还能继续用
    assert_eq!(m.len(), 3);
}

#[test]
fn t4_for_loop_by_value() {
    let m = fruit();
    let mut owned: Vec<(String, u32)> = Vec::new();
    for entry in m {
        owned.push(entry);
    }
    assert_eq!(owned[0], (String::from("apple"), 1));
    assert_eq!(owned[2], (String::from("pear"), 3));
}

#[test]
fn t4_round_trip_through_iterators() {
    // SortedMap -> 迭代器 -> 过滤 -> 再 collect 回 SortedMap
    let big: SortedMap<String, u32> = fruit().into_iter().filter(|(_, n)| *n >= 2).collect();
    let keys: Vec<&str> = big.keys().map(String::as_str).collect();
    assert_eq!(keys, vec!["mango", "pear"]);
}

// ---------------------------------------------------------------
// TODO 5: Index / Debug
// ---------------------------------------------------------------
#[test]
fn t5_index() {
    let m = fruit();
    assert_eq!(m["apple"], 1);
    assert_eq!(m["pear"], 3);
    let nums: SortedMap<u8, char> = [(1, 'a'), (2, 'b')].into_iter().collect();
    assert_eq!(nums[&2], 'b');
}

#[test]
#[should_panic]
fn t5_index_panics_on_missing_key() {
    let m = fruit();
    let _ = m["durian"];
}

#[test]
fn t5_debug() {
    assert_eq!(format!("{:?}", fruit()), r#"{"apple": 1, "mango": 2, "pear": 3}"#);
    let empty: SortedMap<i32, i32> = SortedMap::new();
    assert_eq!(format!("{empty:?}"), "{}");
    let nested: SortedMap<i32, Vec<bool>> = [(2, vec![]), (1, vec![true])].into_iter().collect();
    assert_eq!(format!("{nested:?}"), "{1: [true], 2: []}");
}
