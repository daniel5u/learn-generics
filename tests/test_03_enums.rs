use learn_generics::s03_enums::Maybe::{self, Just, Nothing};
use learn_generics::s03_enums::Outcome::{self, Bad, Good};

// ---------------------------------------------------------------
// TODO 1: 定义 + is_just / unwrap_or
// ---------------------------------------------------------------
#[test]
fn t1_is_just() {
    assert!(Just(1).is_just());
    assert!(!Maybe::<i32>::Nothing.is_just());
}

#[test]
fn t1_unwrap_or() {
    assert_eq!(Just(3).unwrap_or(0), 3);
    assert_eq!(Nothing.unwrap_or(0), 0);
    assert_eq!(Nothing.unwrap_or(String::from("fallback")), "fallback");
}

// ---------------------------------------------------------------
// TODO 2: map / and_then
// ---------------------------------------------------------------
#[test]
fn t2_map_transforms_the_value() {
    assert_eq!(Just(2).map(|x| x * 10), Just(20));
    assert_eq!(Maybe::<i32>::Nothing.map(|x| x * 10), Nothing);
}

#[test]
fn t2_map_can_change_the_type() {
    let m: Maybe<String> = Just(7).map(|x| format!("n={x}"));
    assert_eq!(m, Just(String::from("n=7")));
    let len: Maybe<usize> = Just("hello").map(|s| s.len());
    assert_eq!(len, Just(5));
}

#[test]
fn t2_map_does_not_call_f_on_nothing() {
    let mut called = false;
    let m: Maybe<i32> = Maybe::<i32>::Nothing.map(|x| {
        called = true;
        x
    });
    assert_eq!(m, Nothing);
    assert!(!called);
}

fn half(x: i32) -> Maybe<i32> {
    if x % 2 == 0 { Just(x / 2) } else { Nothing }
}

#[test]
fn t2_and_then_chains_fallible_steps() {
    assert_eq!(Just(8).and_then(half), Just(4));
    assert_eq!(Just(8).and_then(half).and_then(half).and_then(half), Just(1));
    // 6 -> 3 -> 3 是奇数，失败；之后的步骤不再执行
    assert_eq!(Just(6).and_then(half).and_then(half).and_then(half), Nothing);
    assert_eq!(Nothing.and_then(half), Nothing);
}

// ---------------------------------------------------------------
// TODO 3: ok_or + Outcome
// ---------------------------------------------------------------
#[test]
fn t3_ok_or() {
    let good: Outcome<i32, &str> = Just(1).ok_or("missing");
    assert_eq!(good, Good(1));
    let bad: Outcome<i32, &str> = Nothing.ok_or("missing");
    assert_eq!(bad, Bad("missing"));
}

#[test]
fn t3_outcome_map_only_touches_good() {
    let g: Outcome<i32, String> = Good(2);
    assert_eq!(g.map(|x| x + 1), Good(3));
    let b: Outcome<i32, String> = Bad(String::from("boom"));
    assert_eq!(b.map(|x| x + 1), Bad(String::from("boom")));
}

#[test]
fn t3_outcome_map_err_only_touches_bad() {
    let b: Outcome<i32, &str> = Bad("boom");
    let b2: Outcome<i32, usize> = b.map_err(|e| e.len());
    assert_eq!(b2, Bad(4));
    let g: Outcome<i32, &str> = Good(2);
    assert_eq!(g.map_err(|e| e.len()), Good(2));
}

#[test]
fn t3_outcome_ok() {
    let g: Outcome<i32, &str> = Good(2);
    assert_eq!(g.ok(), Just(2));
    let b: Outcome<i32, &str> = Bad("boom");
    assert_eq!(b.ok(), Nothing);
}

// ---------------------------------------------------------------
// TODO 4: 和 Option 互转
// ---------------------------------------------------------------
#[test]
fn t4_from_option() {
    assert_eq!(Maybe::from(Some(3)), Just(3));
    assert_eq!(Maybe::<i32>::from(None), Nothing);
    // 实现 From 之后 into() 自动可用
    let m: Maybe<&str> = Some("x").into();
    assert_eq!(m, Just("x"));
}

#[test]
fn t4_into_option() {
    let o: Option<i32> = Just(3).into();
    assert_eq!(o, Some(3));
    assert_eq!(Option::<i32>::from(Nothing), None);
}

// ---------------------------------------------------------------
// TODO 5: flatten
// ---------------------------------------------------------------
#[test]
fn t5_flatten() {
    assert_eq!(Just(Just(1)).flatten(), Just(1));
    assert_eq!(Just(Maybe::<i32>::Nothing).flatten(), Nothing);
    assert_eq!(Maybe::<Maybe<i32>>::Nothing.flatten(), Nothing);
}

#[test]
fn t5_flatten_after_map() {
    // map 一个"会失败"的函数会得到嵌套的 Maybe，flatten 把它压平 —— 这正是 and_then 做的事
    let nested: Maybe<Maybe<i32>> = Just(8).map(half);
    assert_eq!(nested.flatten(), Just(8).and_then(half));
}
