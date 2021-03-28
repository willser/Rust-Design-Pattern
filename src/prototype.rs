#[derive(Default, Clone)]
struct Prototype {
    age: i32,
    name: String,
    sex: i32,
    address: String,
    email: String,
}

#[test]
fn test() {
    let mut prototype = Prototype::default();

    prototype.address = String::from("dalian");

    let prototype_clone = prototype.clone();
    assert_eq!(prototype.address, prototype_clone.address);
}

