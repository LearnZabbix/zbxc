

//use oping::{Ping, PingResult};
//
//pub fn do_pings() -> PingResult<()> {
//    let mut ping = Ping::new();
//    //try!(ping.set_timeout(3.0));  // timeout of 5.0 seconds
//    ping.set_timeout(3.0)?; // timeout of 5.0 seconds
//    ping.add_host("localhost")?; // fails here if socket can't be created
//                                 //    ping.add_host("other_host")?;
//    ping.add_host("::1")?; // IPv4 / IPv6 addresses OK
//                           //    xping.add_host("1.2.3.4")?;
//    let responses = ping.send()?;
//    for resp in responses {
//        if resp.dropped > 0 {
//            println!("No response from host: {}", resp.hostname);
//        } else {
//            println!(
//                "Response from host {} (address {}): latency {} ms",
//                resp.hostname, resp.address, resp.latency_ms
//            );
//            println!("    all details: {:?}", resp);
//        }
//    }
//    Ok(())
//}

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

//pub fn add(a: i32, b: i32) -> i32 {
//    a + b
//}
//
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


use serde_json::{Result, Value};

pub fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
