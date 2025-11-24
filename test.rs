// test.rs - complex multi-module example

// External imports
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

// Constants and statics
const DEFAULT_CAPACITY: usize = 16;
static mut GLOBAL_STATE: u32 = 0;

// Type aliases
type MyResult<T> = Result<T, String>;
type MyMap = HashMap<String, i32>;

// Traits
trait Processor<T> {
    fn process(&self, value: T) -> T;
    fn info(&self) -> String {
        "Processor trait".to_string()
    }
}

// Nested modules
mod utilities {
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 { a + b }
        pub fn sub(a: i32, b: i32) -> i32 { a - b }
    }

    pub mod strings {
        pub fn capitalize(s: &str) -> String { s.to_uppercase() }
    }
}

// Structs
#[derive(Debug, Clone)]
struct DataHolder<T> {
    pub values: Vec<T>,
}

impl<T: Clone> DataHolder<T> {
    fn new() -> Self { Self { values: Vec::new() } }
    fn add(&mut self, item: T) { self.values.push(item); }
    fn merge(&mut self, other: &Self) { self.values.extend_from_slice(&other.values); }
}

// Generic impl with trait bounds
impl<T: Debug> Processor<T> for DataHolder<T> {
    fn process(&self, value: T) -> T {
        println!("Processing: {:?}", value);
        value
    }
}

// Async function example
async fn fetch_data(url: &str) -> MyResult<String> {
    Ok(format!("Data from {}", url))
}

// Enums
enum Status {
    Ok,
    Error(String),
    Pending(u32),
}

// Macro
macro_rules! create_tuple {
    ($a:expr, $b:expr) => { ($a, $b) };
}

// Nested struct and module combination
mod network {
    pub struct Request {
        pub endpoint: String,
        pub payload: Vec<u8>,
    }

    impl Request {
        pub fn new(endpoint: &str) -> Self {
            Self { endpoint: endpoint.to_string(), payload: Vec::new() }
        }

        pub fn send(&self) -> super::Status {
            println!("Sending request to {}", self.endpoint);
            super::Status::Ok
        }
    }
}

// Functions
fn calculate_sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn print_status(status: &Status) {
    match status {
        Status::Ok => println!("All good"),
        Status::Error(e) => println!("Error: {}", e),
        Status::Pending(n) => println!("Pending: {}", n),
    }
}

// Main test execution
fn main() {
    let mut holder = DataHolder::new();
    holder.add(10);
    holder.add(20);
    println!("{:?}", holder);

    let sum = calculate_sum(&[1, 2, 3, 4]);
    println!("Sum: {}", sum);

    let req = network::Request::new("https://example.com");
    print_status(&req.send());

    let tup = create_tuple!(5, "hello");
    println!("Tuple: {:?}", tup);

    let capitalized = utilities::strings::capitalize("hello world");
    println!("Capitalized: {}", capitalized);
}
