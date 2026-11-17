use learn_generics::s08_const_phantom::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

// ---------------------------------------------------------------
// TODO 1: first_n
// ---------------------------------------------------------------
#[test]
fn t1_first_n_infers_n_from_the_return_type() {
    let a: [i32; 3] = first_n(&[1, 2, 3, 4, 5]);
    assert_eq!(a, [1, 2, 3]);
    let b: [char; 1] = first_n(&['x', 'y']);
    assert_eq!(b, ['x']);
}

#[test]
fn t1_first_n_pads_with_default() {
    assert_eq!(first_n::<u8, 4>(&[7]), [7, 0, 0, 0]);
    assert_eq!(first_n::<bool, 2>(&[]), [false, false]);
    assert_eq!(first_n::<&str, 3>(&["a", "b"]), ["a", "b", ""]);
}

#[test]
fn t1_first_n_zero_length() {
    let empty: [f64; 0] = first_n(&[1.0, 2.0]);
    assert_eq!(empty.len(), 0);
}

// ---------------------------------------------------------------
// TODO 2: Matrix
// ---------------------------------------------------------------
#[test]
fn t2_shape_comes_from_the_type() {
    let m: Matrix<2, 3> = Matrix::zeros();
    assert_eq!(m.shape(), (2, 3));
    assert_eq!(m.data, [[0.0; 3]; 2]);
    // R 和 C 也可以从传给 new 的数组推断出来
    let n = Matrix::new([[1.0], [2.0], [3.0], [4.0]]);
    assert_eq!(n.shape(), (4, 1));
}

#[test]
fn t2_transpose_swaps_the_dimensions() {
    let m = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let t: Matrix<3, 2> = m.transpose();
    assert_eq!(t, Matrix::new([[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]));
    assert_eq!(t.transpose(), m);
}

#[test]
fn t2_mul() {
    let a = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]); // 2x3
    let b = Matrix::new([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]); // 3x2
    let c: Matrix<2, 2> = a.mul(&b);
    assert_eq!(c, Matrix::new([[58.0, 64.0], [139.0, 154.0]]));
    // 反过来乘是 3x3
    let d: Matrix<3, 3> = b.mul(&a);
    assert_eq!(d.data[0], [39.0, 54.0, 69.0]);
}

#[test]
fn t2_mul_chain_tracks_dimensions() {
    let a: Matrix<1, 2> = Matrix::new([[1.0, 2.0]]);
    let b: Matrix<2, 3> = Matrix::new([[1.0, 0.0, 1.0], [0.0, 1.0, 1.0]]);
    let c: Matrix<3, 1> = Matrix::new([[1.0], [1.0], [1.0]]);
    // (1x2 · 2x3) · 3x1 = 1x1，中间结果的类型 Matrix<1, 3> 是编译器自己推出来的
    let out = a.mul(&b).mul(&c);
    assert_eq!(out.shape(), (1, 1));
    assert_eq!(out.data, [[6.0]]);
}

// ---------------------------------------------------------------
// TODO 3: Length<U>
// ---------------------------------------------------------------
#[test]
fn t3_add_same_unit() {
    let total: Length<Meters> = Length::<Meters>::new(1.5) + Length::new(2.0);
    assert!(close(total.value(), 3.5));
    let feet = Length::<Feet>::new(1.0) + Length::<Feet>::new(1.0);
    assert!(close(feet.value(), 2.0));
}

#[test]
fn t3_convert_units() {
    let m: Length<Meters> = Length::<Feet>::new(10.0).to_meters();
    assert!(close(m.value(), 3.048));
    let f: Length<Feet> = Length::<Meters>::new(3.048).to_feet();
    assert!(close(f.value(), 10.0));
    // 换算之后单位一致了，就可以相加
    let sum = Length::<Meters>::new(1.0) + Length::<Feet>::new(1.0).to_meters();
    assert!(close(sum.value(), 1.3048));
}

#[test]
fn t3_length_is_copy_for_any_unit() {
    // Meters 没有实现 Copy / Clone。用 #[derive(Clone, Copy)] 的话，这里编译不过
    let a = Length::<Meters>::new(1.0);
    let b = a;
    let c = a; // a 被用了两次：需要 Copy
    assert!(close((b + c).value(), 2.0));
}

#[test]
fn t3_phantom_data_costs_nothing() {
    assert_eq!(std::mem::size_of::<Length<Meters>>(), std::mem::size_of::<f64>());
}

// ---------------------------------------------------------------
// TODO 4: Door<S>
// ---------------------------------------------------------------
#[test]
fn t4_new_door_is_closed() {
    let door: Door<Closed> = Door::new();
    assert_eq!(door.state(), "closed");
    assert_eq!(door.times_opened(), 0);
}

#[test]
fn t4_open_and_walk_through() {
    let door = Door::new().open();
    assert_eq!(door.state(), "open");
    assert_eq!(door.walk_through(), "walked through");
    assert_eq!(door.times_opened(), 1);
}

#[test]
fn t4_state_transitions_change_the_type() {
    let closed: Door<Closed> = Door::new();
    let open: Door<Open> = closed.open();
    let closed_again: Door<Closed> = open.close();
    assert_eq!(closed_again.state(), "closed");
    let reopened = closed_again.open();
    assert_eq!(reopened.times_opened(), 2);
    assert_eq!(reopened.close().times_opened(), 2);
}

#[test]
fn t4_state_name_is_an_associated_const() {
    assert_eq!(<Open as DoorState>::NAME, "open");
    assert_eq!(Closed::NAME, "closed");
}
