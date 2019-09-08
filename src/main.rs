use explode::Explode;
use explode_derive::Explode;

#[derive(Clone, Explode)]
struct MyStruct {
    a: u32,
    b: String,
}

#[derive(Clone, Explode)]
struct MyTupleStruct(u32, String);

fn main() {
    let s = MyStruct {
        a: 42,
        b: String::from("42"),
    };
    let t: (_, _) = s.clone().into();
    assert_eq!(t, (42, String::from("42")));
    assert_eq!(s.explode(), (42, String::from("42")));

    let s = MyTupleStruct(42, String::from("42"));
    let t: (_, _) = s.clone().into();
    assert_eq!(t, (42, String::from("42")));
    assert_eq!(s.explode(), (42, String::from("42")));
}
