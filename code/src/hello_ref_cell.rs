use std::cell::RefCell;

struct foo_1{
    id: u32,
    username: RefCell<String>,
    active: RefCell<bool>,
}

#[test]
fn test_1(){
    let foo : foo_1 = foo_1 { id: 10, username: RefCell::new(String::from("A")) , active: RefCell::new(true) };
    *foo.username.borrow_mut() = String::from("C");
    assert_eq!(foo.username.into_inner(),String::from("C"));
}