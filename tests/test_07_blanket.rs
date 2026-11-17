use std::fmt;

use learn_generics::s07_blanket::*;

// ---------------------------------------------------------------
// TODO 1: blanket impl
// ---------------------------------------------------------------
#[test]
fn t1_shout_for_builtin_types() {
    assert_eq!(42.shout(), "42!");
    assert_eq!("hi".shout(), "HI!");
    assert_eq!(String::from("Rust").shout(), "RUST!");
    assert_eq!('x'.shout(), "X!");
    assert_eq!(true.shout(), "TRUE!");
    assert_eq!(1.5.shout(), "1.5!");
}

#[test]
fn t1_shout_for_a_type_that_did_not_exist_when_you_wrote_the_impl() {
    struct Temperature(i32);
    impl fmt::Display for Temperature {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} degrees", self.0)
        }
    }
    // Temperature 只实现了 Display，Shout 是 blanket impl 自动给的
    assert_eq!(Temperature(30).shout(), "30 DEGREES!");
}

// ---------------------------------------------------------------
// TODO 2: 扩展 trait
// ---------------------------------------------------------------
#[test]
fn t2_tally_if() {
    assert_eq!((1..=10).tally_if(|x| x % 2 == 0), 5);
    assert_eq!(vec!["a", "bb", "ccc"].into_iter().tally_if(|s| s.len() > 1), 2);
    assert_eq!("Hello World".chars().tally_if(|c| c.is_uppercase()), 2);
    assert_eq!(std::iter::empty::<i32>().tally_if(|_| true), 0);
}

#[test]
fn t2_tally_if_works_on_adapters_and_incomparable_items() {
    // 元素类型不能比较（没有 PartialOrd），tally_if 仍然可用
    struct Job {
        done: bool,
    }
    let jobs = vec![Job { done: true }, Job { done: false }, Job { done: true }];
    assert_eq!(jobs.iter().tally_if(|j| j.done), 2);
    // 接在别的适配器后面
    assert_eq!((1..100).map(|x| x * x).filter(|x| x % 2 == 1).tally_if(|x| *x < 50), 4);
}

#[test]
fn t2_min_max() {
    assert_eq!(vec![3, 1, 4, 1, 5, 9, 2, 6].into_iter().min_max(), Some((1, 9)));
    assert_eq!([2.5, -1.0, 7.25].into_iter().min_max(), Some((-1.0, 7.25)));
    assert_eq!(std::iter::once(7).min_max(), Some((7, 7)));
    assert_eq!(std::iter::empty::<i32>().min_max(), None);
}

#[test]
fn t2_min_max_with_non_copy_items() {
    let words = vec![String::from("pear"), String::from("apple"), String::from("zebra")];
    let (lo, hi) = words.into_iter().min_max().unwrap();
    assert_eq!((lo.as_str(), hi.as_str()), ("apple", "zebra"));
}

// ---------------------------------------------------------------
// TODO 3: newtype
// ---------------------------------------------------------------
#[test]
fn t3_wrapper_display() {
    assert_eq!(Wrapper(vec![1, 2, 3]).to_string(), "[1, 2, 3]");
    assert_eq!(Wrapper(vec!["a"]).to_string(), "[a]");
    assert_eq!(Wrapper(Vec::<i32>::new()).to_string(), "[]");
    assert_eq!(format!("got {}", Wrapper(vec![0.5, 1.5])), "got [0.5, 1.5]");
}

#[test]
fn t3_wrapper_derefs_to_vec() {
    let w = Wrapper(vec![10, 20, 30]);
    // 这些都是 Vec（以及切片）的方法，通过 Deref 自动可用
    assert_eq!(w.len(), 3);
    assert_eq!(w[1], 20);
    assert_eq!(w.iter().sum::<i32>(), 60);
    assert!(w.contains(&30));
}

#[test]
fn t3_blanket_impls_compose() {
    // Wrapper<i32>: Display  =>  Wrapper<i32>: Shout（来自 TODO 1 的 blanket impl）
    assert_eq!(Wrapper(vec!['a', 'b']).shout(), "[A, B]!");
}

// ---------------------------------------------------------------
// TODO 4: 层层组合的 impl
// ---------------------------------------------------------------
#[test]
fn t4_describe_leaves() {
    assert_eq!(7.describe(), "7");
    assert_eq!(true.describe(), "yes");
    assert_eq!(false.describe(), "no");
}

#[test]
fn t4_describe_option_and_vec() {
    assert_eq!(Some(3).describe(), "some(3)");
    assert_eq!(None::<i32>.describe(), "none");
    assert_eq!(vec![1, 2, 3].describe(), "[1, 2, 3]");
    assert_eq!(Vec::<bool>::new().describe(), "[]");
}

#[test]
fn t4_describe_nested() {
    assert_eq!(vec![Some(true), None, Some(false)].describe(), "[some(yes), none, some(no)]");
    assert_eq!(Some(vec![vec![1], vec![], vec![2, 3]]).describe(), "some([[1], [], [2, 3]])");
    assert_eq!(Some(Some(None::<i32>)).describe(), "some(some(none))");
}
