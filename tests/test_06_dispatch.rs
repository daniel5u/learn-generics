use learn_generics::s06_dispatch::*;

fn dog(name: &str) -> Dog {
    Dog { name: name.to_string() }
}

fn cat(name: &str) -> Cat {
    Cat { name: name.to_string() }
}

// ---------------------------------------------------------------
// TODO 1: 单态化
// ---------------------------------------------------------------
#[test]
fn t1_type_name_of() {
    assert_eq!(type_name_of(&1i32), "i32");
    assert_eq!(type_name_of(&1.5f64), "f64");
    assert_eq!(type_name_of(&"hi"), "&str");
    assert!(type_name_of(&dog("Rex")).ends_with("Dog"));
    // 泛型类型的名字里带着它的类型参数：Vec<u8> 和 Vec<bool> 是两个不同的类型
    assert!(type_name_of(&vec![1u8]).ends_with("Vec<u8>"));
    assert!(type_name_of(&vec![true]).ends_with("Vec<bool>"));
}

#[test]
fn t1_chorus_static() {
    let dogs = [dog("Rex"), dog("Fido")];
    assert_eq!(chorus_static(&dogs), vec!["Rex: Woof!", "Fido: Woof!"]);
    // 同一个泛型函数，换一种类型再用一次（编译器会再生成一份）
    let cats = vec![cat("Tom")];
    assert_eq!(chorus_static(&cats), vec!["Tom: Meow!"]);
}

// ---------------------------------------------------------------
// TODO 2: 动态分发
// ---------------------------------------------------------------
#[test]
fn t2_chorus_dyn_mixed_types() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat("Tom")), Box::new(dog("Rex"))];
    assert_eq!(chorus_dyn(&animals), vec!["Tom: Meow!", "Rex: Woof!"]);
    assert_eq!(chorus_dyn(&[]), Vec::<String>::new());
}

#[test]
fn t2_zoo() {
    let z = zoo();
    assert_eq!(z.len(), 3);
    let names: Vec<String> = z.iter().map(|a| a.name()).collect();
    assert_eq!(names, vec!["Rex", "Tom", "Fido"]);
    assert_eq!(chorus_dyn(&z), vec!["Rex: Woof!", "Tom: Meow!", "Fido: Woof!"]);
}

// ---------------------------------------------------------------
// TODO 3: 运行时决定类型
// ---------------------------------------------------------------
#[test]
fn t3_make_animal() {
    assert_eq!(make_animal("dog", "Rex").unwrap().speak(), "Rex: Woof!");
    assert_eq!(make_animal("cat", "Tom").unwrap().speak(), "Tom: Meow!");
    assert!(make_animal("dragon", "Smaug").is_none());
}

#[test]
fn t3_make_animal_from_runtime_data() {
    let spec = "cat:Tom,dog:Rex,fish:Nemo,cat:Kitty";
    let animals: Vec<Box<dyn Animal>> = spec
        .split(',')
        .filter_map(|item| {
            let (kind, name) = item.split_once(':')?;
            make_animal(kind, name)
        })
        .collect();
    assert_eq!(chorus_dyn(&animals), vec!["Tom: Meow!", "Rex: Woof!", "Kitty: Meow!"]);
}

// ---------------------------------------------------------------
// TODO 4: 胖指针
// ---------------------------------------------------------------
#[test]
fn t4_pointer_sizes() {
    let ptr = std::mem::size_of::<usize>();
    let (thin, fat_ref, fat_box) = pointer_sizes();
    assert_eq!(thin, ptr, "&Dog 是普通指针");
    assert_eq!(fat_ref, 2 * ptr, "&dyn Animal = 数据指针 + vtable 指针");
    assert_eq!(fat_box, 2 * ptr, "Box<dyn Animal> 也一样");
}

// ---------------------------------------------------------------
// TODO 5: ?Sized
// ---------------------------------------------------------------
#[test]
fn t5_speak_twice_static() {
    assert_eq!(speak_twice(&dog("Rex")), "Rex: Woof! Rex: Woof!");
    assert_eq!(speak_twice(&cat("Tom")), "Tom: Meow! Tom: Meow!");
}

#[test]
fn t5_speak_twice_dyn() {
    let boxed: Box<dyn Animal> = Box::new(cat("Tom"));
    // &*boxed 的类型是 &dyn Animal，所以这里 T = dyn Animal
    assert_eq!(speak_twice(&*boxed), "Tom: Meow! Tom: Meow!");
    for animal in zoo() {
        let twice = speak_twice(animal.as_ref());
        assert_eq!(twice, format!("{0} {0}", animal.speak()));
    }
}
