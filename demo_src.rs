// demo_src.rs
use std::collections::HashMap;

const GREETING: &str = "Hello, Walt!";

fn main() {
    let mut map = HashMap::new();
    map.insert("version", 1);
    println!("{} (version: {})", GREETING, map["version"]);
}
