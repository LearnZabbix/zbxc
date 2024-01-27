//use fake::{Fake, Faker};
//use log::LevelFilter;
//
//pub fn init_logging() {
//    let _ = env_logger::builder()
//        .filter_level(LevelFilter::Debug)
//        .is_test(true)
//        .try_init();
//}
//
//pub fn get_random_string() -> String {
//    Faker.fake::<String>()
//}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn greeting(name: String) -> String {
    let hello = String::from("Hello, ");
    let greeting = format!("{hello}{name}!");
    greeting
}

pub fn hello_world() -> String {
    let greeting = String::from("Hello, World!");
    greeting
}
