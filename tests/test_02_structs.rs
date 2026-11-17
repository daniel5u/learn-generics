use learn_generics::s02_structs::*;

// ---------------------------------------------------------------
// TODO 1: Stack<T> 的基本方法
// ---------------------------------------------------------------
#[test]
fn t1_new_stack_is_empty() {
    let s: Stack<i32> = Stack::new();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.peek(), None);
}

#[test]
fn t1_push_pop_is_last_in_first_out() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.len(), 3);
    assert_eq!(s.peek(), Some(&3));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    assert!(s.is_empty());
}

#[test]
fn t1_stack_holds_any_type() {
    // 一个既不能打印、也不能比较、也不能复制的类型
    struct Opaque;
    let mut s = Stack::new();
    s.push(Opaque);
    s.push(Opaque);
    assert_eq!(s.len(), 2);
    assert!(s.pop().is_some());

    let mut words: Stack<String> = Stack::new();
    words.push(String::from("hi"));
    assert_eq!(words.peek().map(|w| w.as_str()), Some("hi"));
}

// ---------------------------------------------------------------
// TODO 2: 条件方法
// ---------------------------------------------------------------
#[test]
fn t2_render() {
    let mut s = Stack::new();
    assert_eq!(s.render(), "[]");
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.render(), "[1, 2, 3]");

    let mut words = Stack::new();
    words.push("a");
    words.push("b");
    assert_eq!(words.render(), "[a, b]");
}

#[test]
fn t2_max() {
    let mut s = Stack::new();
    assert_eq!(s.max(), None);
    s.push(2.5);
    s.push(9.0);
    s.push(-1.0);
    assert_eq!(s.max(), Some(&9.0));
    // max 只是借用，栈没有变
    assert_eq!(s.len(), 3);
}

// ---------------------------------------------------------------
// TODO 3: 只属于 Stack<f64> 的方法
// ---------------------------------------------------------------
#[test]
fn t3_sum_of_floats() {
    let mut s = Stack::new();
    assert_eq!(s.sum(), 0.0);
    s.push(1.5);
    s.push(2.5);
    assert_eq!(s.sum(), 4.0);
}

// ---------------------------------------------------------------
// TODO 4: Default
// ---------------------------------------------------------------
#[test]
fn t4_default_is_empty() {
    let s: Stack<u8> = Stack::default();
    assert!(s.is_empty());
}

#[test]
fn t4_default_does_not_require_t_default() {
    // NoDefault 没有实现 Default。用 #[derive(Default)] 的话，这里编译不过
    struct NoDefault;
    let mut s: Stack<NoDefault> = Stack::default();
    s.push(NoDefault);
    assert_eq!(s.len(), 1);
}

// ---------------------------------------------------------------
// TODO 5: Pair<A, B>
// ---------------------------------------------------------------
#[test]
fn t5_new_and_swap() {
    let p = Pair::new(1, "one");
    assert_eq!(p.first, 1);
    assert_eq!(p.second, "one");
    let q: Pair<&str, i32> = p.swap();
    assert_eq!(q, Pair { first: "one", second: 1 });
}

#[test]
fn t5_mixup() {
    let a = Pair::new(1, 2.5);
    let b = Pair::new("x", 'y');
    let c: Pair<i32, char> = a.mixup(b);
    assert_eq!(c, Pair::new(1, 'y'));
}

#[test]
fn t5_map_first_can_change_the_type() {
    let p = Pair::new(21, "kept");
    assert_eq!(p.clone().map_first(|n| n * 2), Pair::new(42, "kept"));
    // 同一个 Pair<i32, &str>，换一个闭包，C 就变成了 String
    let q: Pair<String, &str> = p.map_first(|n| format!("#{n}"));
    assert_eq!(q.first, "#21");
}

#[test]
fn t5_map_first_accepts_fn_once() {
    // 这个闭包把 owned 移出去了，只能调用一次，所以它只实现 FnOnce
    let owned = String::from("moved");
    let p = Pair::new(1, 2).map_first(move |n| (n, owned));
    assert_eq!(p.first, (1, String::from("moved")));
}
