use std::cell::Cell;

use learn_generics::s09_closures_lifetimes::*;

// ---------------------------------------------------------------
// TODO 1: 接收闭包
// ---------------------------------------------------------------
#[test]
fn t1_apply_twice() {
    assert_eq!(apply_twice(|x| x * 2, 5), 20);
    assert_eq!(apply_twice(|s: String| s + "!", String::from("hi")), "hi!!");
    // 普通函数也可以当闭包传
    assert_eq!(apply_twice(f64::sqrt, 16.0), 2.0);
}

#[test]
fn t1_apply_twice_with_captured_variable() {
    let step = 10;
    assert_eq!(apply_twice(|x| x + step, 1), 21);
}

#[test]
fn t1_call_n_accepts_mutating_closures() {
    let mut count = 0;
    call_n(3, || count += 1);
    assert_eq!(count, 3);

    let mut log = Vec::new();
    call_n(2, || log.push("tick"));
    assert_eq!(log, vec!["tick", "tick"]);

    call_n(0, || panic!("n = 0 时不应该调用"));
}

// ---------------------------------------------------------------
// TODO 2: 返回闭包
// ---------------------------------------------------------------
#[test]
fn t2_make_adder() {
    let add5 = make_adder(5);
    assert_eq!(add5(1), 6);
    assert_eq!(add5(10), 15);
    // 返回的闭包可以再传给别的泛型函数
    assert_eq!(apply_twice(make_adder(100), 1), 201);
}

#[test]
fn t2_make_counter_keeps_its_own_state() {
    let mut a = make_counter();
    let mut b = make_counter();
    assert_eq!(a(), 1);
    assert_eq!(a(), 2);
    assert_eq!(a(), 3);
    assert_eq!(b(), 1, "两个计数器互不影响");
}

#[test]
fn t2_compose() {
    let add_then_double = compose(|x: i32| x + 1, |x: i32| x * 2);
    assert_eq!(add_then_double(3), 8);
    // A = &str, B = usize, C = bool
    let is_long = compose(|s: &str| s.len(), |n: usize| n > 3);
    assert!(is_long("hello"));
    assert!(!is_long("hi"));
}

#[test]
fn t2_compose_chains() {
    let f = compose(compose(make_adder(1), make_adder(2)), |x: i32| x.to_string());
    assert_eq!(f(0), "3");
    assert_eq!(f(7), "10");
}

// ---------------------------------------------------------------
// TODO 3: Lazy
// ---------------------------------------------------------------
#[test]
fn t3_lazy_does_not_run_until_asked() {
    let calls = Cell::new(0);
    let mut lazy = Lazy::new(|| {
        calls.set(calls.get() + 1);
        42
    });
    assert_eq!(calls.get(), 0, "new 不应该执行闭包");
    assert!(!lazy.is_initialized());
    assert_eq!(*lazy.get(), 42);
    assert!(lazy.is_initialized());
    assert_eq!(calls.get(), 1);
}

#[test]
fn t3_lazy_runs_only_once() {
    let calls = Cell::new(0);
    let mut lazy = Lazy::new(|| {
        calls.set(calls.get() + 1);
        String::from("expensive")
    });
    assert_eq!(lazy.get(), "expensive");
    assert_eq!(lazy.get(), "expensive");
    assert_eq!(lazy.get().len(), 9);
    assert_eq!(calls.get(), 1);
}

#[test]
fn t3_lazy_accepts_fn_once() {
    // 这个闭包把 data 整个移交出去，只能调用一次
    let data = vec![1, 2, 3];
    let mut lazy = Lazy::new(move || data);
    assert_eq!(lazy.get(), &vec![1, 2, 3]);
    assert_eq!(lazy.get().len(), 3);
}

// ---------------------------------------------------------------
// TODO 4: 生命周期
// ---------------------------------------------------------------
#[test]
fn t4_longest() {
    assert_eq!(longest("hello", "hi"), "hello");
    assert_eq!(longest("hi", "hello"), "hello");
    assert_eq!(longest("ab", "cd"), "ab");
}

#[test]
fn t4_longest_result_lives_as_long_as_both_inputs() {
    let a = String::from("long string");
    let result;
    {
        let b = String::from("short");
        result = longest(&a, &b).to_string(); // 在 b 销毁前拷贝出来
    }
    assert_eq!(result, "long string");
}

#[test]
fn t4_find() {
    assert_eq!(find("hello world", "world"), Some(Highlight { text: "world", start: 6 }));
    assert_eq!(find("hello world", "o"), Some(Highlight { text: "o", start: 4 }));
    assert_eq!(find("hello world", "xyz"), None);
}

#[test]
fn t4_find_borrows_from_haystack_not_from_needle() {
    let haystack = String::from("the quick brown fox");
    let found = {
        let needle = String::from("quick");
        find(&haystack, &needle)
        // needle 在这里销毁
    };
    // found 仍然可用：它只借用了 haystack。如果签名把 needle 也标成 'a，这个测试编译不过
    let found = found.unwrap();
    assert_eq!(found.text, "quick");
    assert_eq!(found.start, 4);
    // text 真的是 haystack 的一部分（指向同一块内存），而不是拷贝
    assert!(std::ptr::eq(found.text.as_ptr(), haystack[4..].as_ptr()));
}

// ---------------------------------------------------------------
// TODO 5: 综合
// ---------------------------------------------------------------
#[test]
fn t5_best_by() {
    assert_eq!(best_by(&["a", "ccc", "bb"], |s| s.len()), Some(&"ccc"));
    assert_eq!(best_by(&[3i32, -7, 5], |x| x.abs()), Some(&-7));
    assert_eq!(best_by(&[1.5, 0.5], |x| -x), Some(&0.5));
    assert_eq!(best_by::<i32, i32, _>(&[], |x| *x), None);
}

#[test]
fn t5_best_by_first_wins_on_ties_and_returns_a_borrow() {
    let people = vec![
        (String::from("ann"), 30),
        (String::from("bob"), 41),
        (String::from("cat"), 41),
    ];
    let oldest: &(String, u32) = best_by(&people, |p| p.1).unwrap();
    assert_eq!(oldest.0, "bob");
    // 返回的是 people 里那个元素的引用
    assert!(std::ptr::eq(oldest, &people[1]));
}

#[test]
fn t5_box_all() {
    let boxed = box_all(vec![1, 2, 3]);
    let shown: Vec<String> = boxed.iter().map(|b| b.to_string()).collect();
    assert_eq!(shown, vec!["1", "2", "3"]);

    // 不同的 T 装箱之后变成了同一种类型，可以放进同一个 Vec
    let mut mixed = box_all(vec![String::from("owned")]);
    mixed.extend(box_all(vec![1.5]));
    mixed.extend(box_all(vec!["static str"]));
    let shown: Vec<String> = mixed.iter().map(|b| b.to_string()).collect();
    assert_eq!(shown, vec!["owned", "1.5", "static str"]);
}
