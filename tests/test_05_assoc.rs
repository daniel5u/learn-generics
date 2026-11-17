use learn_generics::s05_assoc::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

// ---------------------------------------------------------------
// TODO 1: From / Into
// ---------------------------------------------------------------
#[test]
fn t1_celsius_to_fahrenheit_and_back() {
    let f = Fahrenheit::from(Celsius(100.0));
    assert!(close(f.0, 212.0));
    let c: Celsius = Fahrenheit(32.0).into();
    assert!(close(c.0, 0.0));
    let round_trip: Celsius = Fahrenheit::from(Celsius(36.6)).into();
    assert!(close(round_trip.0, 36.6));
}

#[test]
fn t1_one_type_many_from_impls() {
    // Fahrenheit 同时实现了 From<Celsius> 和 From<f64>
    assert_eq!(Fahrenheit::from(50.0), Fahrenheit(50.0));
    assert!(close(Fahrenheit::from(Celsius(0.0)).0, 32.0));
}

#[test]
fn t1_generic_over_into() {
    assert!(close(fahrenheit_value(Celsius(100.0)), 212.0));
    assert!(close(fahrenheit_value(Fahrenheit(70.0)), 70.0));
    assert!(close(fahrenheit_value(451.0), 451.0));
}

// ---------------------------------------------------------------
// TODO 2: 运算符
// ---------------------------------------------------------------
#[test]
fn t2_add_vectors() {
    let a = Vec2 { x: 1, y: 2 };
    let b = Vec2 { x: 10, y: 20 };
    assert_eq!(a + b, Vec2 { x: 11, y: 22 });
    assert_eq!(Vec2 { x: 0.5, y: 1.5 } + Vec2 { x: 0.5, y: 0.5 }, Vec2 { x: 1.0, y: 2.0 });
}

#[test]
fn t2_add_only_needs_add_not_copy() {
    // String + String 不存在（只有 String + &str），所以用一个自己的非 Copy 类型来测
    #[derive(Debug, PartialEq)]
    struct Text(String);
    impl std::ops::Add for Text {
        type Output = Text;
        fn add(self, rhs: Text) -> Text {
            Text(self.0 + &rhs.0)
        }
    }
    let a = Vec2 { x: Text("a".into()), y: Text("b".into()) };
    let b = Vec2 { x: Text("1".into()), y: Text("2".into()) };
    assert_eq!(a + b, Vec2 { x: Text("a1".into()), y: Text("b2".into()) });
}

#[test]
fn t2_scale_by_scalar() {
    assert_eq!(Vec2 { x: 1, y: -2 } * 3, Vec2 { x: 3, y: -6 });
    assert_eq!(Vec2 { x: 1.5, y: 2.0 } * 2.0, Vec2 { x: 3.0, y: 4.0 });
}

// ---------------------------------------------------------------
// TODO 3: Container
// ---------------------------------------------------------------
#[test]
fn t3_vec_is_a_container() {
    let v = vec!["a", "b", "c"];
    assert_eq!(Container::get(&v, 1), Some(&"b"));
    assert_eq!(Container::get(&v, 3), None);
    assert_eq!(v.size(), 3);
    assert_eq!(Container::first(&v), Some(&"a"));
}

#[test]
fn t3_repeat_is_a_container() {
    let r = Repeat { value: 7, times: 3 };
    assert_eq!(r.size(), 3);
    assert_eq!(r.get(0), Some(&7));
    assert_eq!(r.get(2), Some(&7));
    assert_eq!(r.get(3), None);
    assert_eq!(r.first(), Some(&7));
    assert_eq!(Repeat { value: 7, times: 0 }.first(), None);
}

#[test]
fn t3_last_item() {
    assert_eq!(last_item(&vec![1, 2, 3]), Some(&3));
    assert_eq!(last_item(&Vec::<i32>::new()), None);
    assert_eq!(last_item(&Repeat { value: "x", times: 5 }), Some(&"x"));
    assert_eq!(last_item(&Repeat { value: "x", times: 0 }), None);
}

#[test]
fn t3_contains() {
    assert!(contains(&vec![1, 2, 3], &2));
    assert!(!contains(&vec![1, 2, 3], &9));
    assert!(contains(&Repeat { value: 'z', times: 2 }, &'z'));
    assert!(!contains(&Repeat { value: 'z', times: 0 }, &'z'));
}

// ---------------------------------------------------------------
// TODO 4: Iterator
// ---------------------------------------------------------------
#[test]
fn t4_countdown() {
    let got: Vec<u32> = Countdown(3).collect();
    assert_eq!(got, vec![3, 2, 1]);
    assert_eq!(Countdown(0).next(), None);
}

#[test]
fn t4_countdown_gets_all_iterator_methods_for_free() {
    assert_eq!(Countdown(4).sum::<u32>(), 10);
    let evens: Vec<u32> = Countdown(6).filter(|n| n % 2 == 0).collect();
    assert_eq!(evens, vec![6, 4, 2]);
    let mut seen = Vec::new();
    for n in Countdown(2) {
        seen.push(n);
    }
    assert_eq!(seen, vec![2, 1]);
}

#[test]
fn t4_pairs() {
    let got: Vec<(i32, i32)> = pairs(vec![1, 2, 3, 4]).collect();
    assert_eq!(got, vec![(1, 2), (3, 4)]);
    // 落单的丢掉
    let got: Vec<(char, char)> = pairs("abcde".chars()).collect();
    assert_eq!(got, vec![('a', 'b'), ('c', 'd')]);
    assert_eq!(pairs(Vec::<u8>::new()).next(), None);
}

#[test]
fn t4_pairs_wraps_any_iterator() {
    // 套在我们自己的迭代器外面
    let got: Vec<(u32, u32)> = pairs(Countdown(5)).collect();
    assert_eq!(got, vec![(5, 4), (3, 2)]);
    // 套两层: Item = ((u32, u32), (u32, u32))
    let got: Vec<((u32, u32), (u32, u32))> = pairs(pairs(Countdown(8))).collect();
    assert_eq!(got, vec![((8, 7), (6, 5)), ((4, 3), (2, 1))]);
    // 元素不需要 Copy
    let words = vec![String::from("a"), String::from("b")];
    assert_eq!(pairs(words).next(), Some((String::from("a"), String::from("b"))));
}

// ---------------------------------------------------------------
// TODO 5: sum_all
// ---------------------------------------------------------------
#[test]
fn t5_sum_numbers() {
    assert_eq!(sum_all(vec![1, 2, 3]), 6);
    assert_eq!(sum_all([0.5, 0.25]), 0.75);
    assert_eq!(sum_all(1u64..=100), 5050);
    assert_eq!(sum_all(Vec::<i32>::new()), 0);
}

#[test]
fn t5_sum_our_own_types() {
    assert_eq!(sum_all(Countdown(4)), 10);
    let vs = vec![Vec2 { x: 1, y: 2 }, Vec2 { x: 3, y: 4 }, Vec2 { x: 5, y: 6 }];
    assert_eq!(sum_all(vs), Vec2 { x: 9, y: 12 });
}
