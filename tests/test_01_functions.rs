use learn_generics::s01_functions::*;

// ---------------------------------------------------------------
// TODO 1: swap
// ---------------------------------------------------------------
#[test]
fn t1_swap_two_different_types() {
    assert_eq!(swap((1, "a")), ("a", 1));
    assert_eq!(swap((2.5, true)), (true, 2.5));
}

#[test]
fn t1_swap_works_without_copy() {
    // String 和 Vec 都不是 Copy，swap 只能 move 它们
    let (v, s) = swap((String::from("hello"), vec![1, 2, 3]));
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(s, "hello");
}

// ---------------------------------------------------------------
// TODO 2: largest
// ---------------------------------------------------------------
#[test]
fn t2_largest_integers() {
    assert_eq!(largest(&[1, 5, 3]), Some(&5));
    assert_eq!(largest(&[-7]), Some(&-7));
}

#[test]
fn t2_largest_other_types() {
    assert_eq!(largest(&[1.5, 0.2, 9.75]), Some(&9.75));
    assert_eq!(largest(&['x', 'b', 'q']), Some(&'x'));
    assert_eq!(largest(&["pear", "apple", "zebra"]), Some(&"zebra"));
}

#[test]
fn t2_largest_of_empty_is_none() {
    // 空切片里没有任何信息能告诉编译器 T 是什么，所以要用 turbofish
    assert_eq!(largest::<i32>(&[]), None);
}

#[test]
fn t2_largest_does_not_need_copy_or_clone() {
    // 如果你的签名要求了 T: Copy，这里会编译失败
    let words = vec![String::from("bb"), String::from("c"), String::from("a")];
    let best: Option<&String> = largest(&words);
    assert_eq!(best.unwrap(), "c");
    // words 还能继续用：largest 只是借用
    assert_eq!(words.len(), 3);
}

// ---------------------------------------------------------------
// TODO 3: count_eq
// ---------------------------------------------------------------
#[test]
fn t3_count_eq() {
    assert_eq!(count_eq(&[1, 2, 1, 1], &1), 3);
    assert_eq!(count_eq(&["a", "b", "a"], &"a"), 2);
    assert_eq!(count_eq(&[true, false], &true), 1);
    assert_eq!(count_eq::<char>(&[], &'x'), 0);
}

#[test]
fn t3_count_eq_only_needs_equality() {
    // 这个类型只能判等，不能比大小、不能复制
    #[derive(PartialEq)]
    struct Color(u8, u8, u8);
    let colors = [Color(0, 0, 0), Color(255, 0, 0), Color(0, 0, 0)];
    assert_eq!(count_eq(&colors, &Color(0, 0, 0)), 2);
}

// ---------------------------------------------------------------
// TODO 4: parse_all
// ---------------------------------------------------------------
#[test]
fn t4_parse_all_with_turbofish() {
    assert_eq!(parse_all::<i32>(&["1", "x", "3"]), vec![1, 3]);
    assert_eq!(parse_all::<bool>(&["true", "nope", "false"]), vec![true, false]);
}

#[test]
fn t4_parse_all_with_annotation() {
    let floats: Vec<f64> = parse_all(&["1.5", "2", "abc"]);
    assert_eq!(floats, vec![1.5, 2.0]);
}

#[test]
fn t4_same_input_different_t_gives_different_result() {
    let inputs = ["7", "-7", "7.5"];
    assert_eq!(parse_all::<u8>(&inputs), vec![7]);
    assert_eq!(parse_all::<i32>(&inputs), vec![7, -7]);
    assert_eq!(parse_all::<f32>(&inputs), vec![7.0, -7.0, 7.5]);
}

// ---------------------------------------------------------------
// TODO 5: describe_order
// ---------------------------------------------------------------
#[test]
fn t5_describe_order_numbers() {
    assert_eq!(describe_order(&1, &2), "1 < 2");
    assert_eq!(describe_order(&2, &1), "2 > 1");
    assert_eq!(describe_order(&1, &1), "1 == 1");
}

#[test]
fn t5_describe_order_uses_debug_format() {
    assert_eq!(describe_order(&"a", &"b"), "\"a\" < \"b\"");
    assert_eq!(describe_order(&vec![1, 2], &vec![1]), "[1, 2] > [1]");
}

#[test]
fn t5_describe_order_incomparable() {
    assert_eq!(describe_order(&f64::NAN, &f64::NAN), "NaN ? NaN");
    assert_eq!(describe_order(&1.0, &f64::NAN), "1.0 ? NaN");
}
