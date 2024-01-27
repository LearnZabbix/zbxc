#[test]
fn hello_world_test() {
    let want = String::from("Hello, World!");
    let result = crate::functions::hello_world();
    assert_eq!(want, result);
}
#[test]
fn greeting_test() {
    let want = String::from("Hello, Rusty!");
    let name = String::from("Rusty");
    let result = crate::functions::greeting(name);
    assert_eq!(want, result);
}
