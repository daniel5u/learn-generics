use std::collections::HashMap;
use std::f64::consts::PI;

use learn_generics::s04_traits::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

// ---------------------------------------------------------------
// TODO 1: 实现 Shape + 默认方法
// ---------------------------------------------------------------
#[test]
fn t1_area_and_name() {
    let c = Circle { r: 2.0 };
    let r = Rect { w: 2.0, h: 3.0 };
    assert!(close(c.area(), 4.0 * PI));
    assert!(close(r.area(), 6.0));
    assert_eq!(c.name(), "circle");
    assert_eq!(r.name(), "rect");
}

#[test]
fn t1_default_describe() {
    assert_eq!(Circle { r: 1.0 }.describe(), "circle (area 3.14)");
    assert_eq!(Rect { w: 2.0, h: 3.0 }.describe(), "rect (area 6.00)");
}

#[test]
fn t1_default_method_works_for_types_defined_elsewhere() {
    // 一个在测试里才定义的新形状：只实现两个必需方法，describe 自动可用
    struct Triangle;
    impl Shape for Triangle {
        fn area(&self) -> f64 {
            0.5
        }
        fn name(&self) -> String {
            "triangle".to_string()
        }
    }
    assert_eq!(Triangle.describe(), "triangle (area 0.50)");
}

// ---------------------------------------------------------------
// TODO 2: 三种写法
// ---------------------------------------------------------------
#[test]
fn t2_total_area() {
    let rects = [Rect { w: 1.0, h: 1.0 }, Rect { w: 2.0, h: 3.0 }];
    assert!(close(total_area(&rects), 7.0));
    let circles = vec![Circle { r: 1.0 }, Circle { r: 1.0 }];
    assert!(close(total_area(&circles), 2.0 * PI));
    assert!(close(total_area::<Rect>(&[]), 0.0));
}

#[test]
fn t2_bigger_name_accepts_two_different_types() {
    let c = Circle { r: 1.0 }; // 3.14
    let small = Rect { w: 1.0, h: 1.0 };
    let big = Rect { w: 10.0, h: 10.0 };
    assert_eq!(bigger_name(&c, &small), "circle");
    assert_eq!(bigger_name(&c, &big), "rect");
    assert_eq!(bigger_name(&small, &big), "rect");
}

#[test]
fn t2_larger_returns_the_same_type() {
    let a = Rect { w: 1.0, h: 1.0 };
    let b = Rect { w: 2.0, h: 2.0 };
    // 返回值的类型就是 Rect，所以可以访问 .w
    let winner: Rect = larger(a.clone(), b.clone());
    assert_eq!(winner, b);
    assert_eq!(winner.w, 2.0);
    // 一样大时返回第一个
    let c = Rect { w: 4.0, h: 1.0 };
    assert_eq!(larger(b.clone(), c), b);
}

// ---------------------------------------------------------------
// TODO 3: where 子句
// ---------------------------------------------------------------
#[test]
fn t3_summarize_sorted_by_key() {
    let mut shapes = HashMap::new();
    shapes.insert("b", Rect { w: 2.0, h: 3.0 });
    shapes.insert("c", Rect { w: 1.0, h: 1.0 });
    shapes.insert("a", Rect { w: 0.5, h: 0.5 });
    assert_eq!(
        summarize(&shapes),
        vec!["a: rect (area 0.25)", "b: rect (area 6.00)", "c: rect (area 1.00)"]
    );
}

#[test]
fn t3_summarize_other_key_and_value_types() {
    let mut shapes: HashMap<u32, Circle> = HashMap::new();
    shapes.insert(10, Circle { r: 1.0 });
    shapes.insert(2, Circle { r: 2.0 });
    // 数字 key 按数值排序（2 < 10），而不是按字符串排序
    assert_eq!(summarize(&shapes), vec!["2: circle (area 12.57)", "10: circle (area 3.14)"]);
}

// ---------------------------------------------------------------
// TODO 4: 返回 impl Trait
// ---------------------------------------------------------------
#[test]
fn t4_make_square() {
    let sq = make_square(3.0);
    // sq 的具体类型被隐藏了，只能调用 Shape 的方法
    assert!(close(sq.area(), 9.0));
    // 但它依然可以传给任何接受 Shape 的泛型函数
    assert!(close(total_area(&[sq]), 9.0));
}

#[test]
fn t4_scaled_areas() {
    let rects = [Rect { w: 1.0, h: 2.0 }, Rect { w: 3.0, h: 4.0 }];
    let got: Vec<f64> = scaled_areas(&rects, 10.0).collect();
    assert_eq!(got, vec![20.0, 120.0]);
    // 返回的是真正的迭代器，可以继续接适配器
    assert_eq!(scaled_areas(&rects, 1.0).map(|a| a as i32).max(), Some(12));
}

// ---------------------------------------------------------------
// TODO 5: 元组的条件实现
// ---------------------------------------------------------------
#[test]
fn t5_tuple_is_a_shape() {
    let combo = (Circle { r: 1.0 }, Rect { w: 2.0, h: 3.0 });
    assert!(close(combo.area(), PI + 6.0));
    assert_eq!(combo.name(), "circle+rect");
    assert_eq!(combo.describe(), "circle+rect (area 9.14)");
}

#[test]
fn t5_nested_tuples_and_generic_functions() {
    let unit = Rect { w: 1.0, h: 1.0 };
    let nested = ((unit.clone(), unit.clone()), (unit.clone(), unit.clone()));
    assert!(close(nested.area(), 4.0));
    assert_eq!(nested.name(), "rect+rect+rect+rect");

    let pairs = [(unit.clone(), unit.clone()), (unit.clone(), unit.clone())];
    assert!(close(total_area(&pairs), 4.0));
    assert_eq!(bigger_name(&nested, &unit), "rect+rect+rect+rect");
}
