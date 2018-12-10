extern crate kvclient;
use kvclient::client::*;
use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");

    let mut servers: Vec<String> = Vec::new();
    servers.push(String::from("127.0.0.1:8081"));
    servers.push(String::from("127.0.0.1:8083"));
    servers.push(String::from("127.0.0.1:8085"));
    servers.push(String::from("127.0.0.1:8087"));
    servers.push(String::from("127.0.0.1:8089"));

    let mut client = Client::new(servers);
    println!("New a client");
    for i in 0..100 {
        client.put(format!("key{}", i),format!("value{}", i));
        thread::sleep(Duration::from_millis(100));
    }
    for i in 0..100 {
        println!("get : {}",client.get(format!("key{}", i)));
    }
}
