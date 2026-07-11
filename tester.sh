#!/usr/bin/env bash
# End-to-end round-trip fidelity gate for walt.
#
# Encodes a complex Rust source file to .ars, decodes it back, and requires
# a BYTE-IDENTICAL round trip. Any difference is a test failure — there is
# no partial-credit "accuracy" metric here.
set -euo pipefail

cd "$(dirname "$0")"
cargo build --quiet

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

TEST_FILE="$TMP_DIR/test.rs"
ENCODED_FILE="$TMP_DIR/test.ars"
DECODED_FILE="$TMP_DIR/decoded_test.rs"

cat > "$TEST_FILE" << 'EOF'
// --- START test.rs ---
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

const DEFAULT_CAPACITY: usize = 16;
static mut GLOBAL_STATE: u32 = 0;

type MyResult<T> = Result<T, String>;
type MyMap = HashMap<String, i32>;

trait Processor<T> {
    fn process(&self, value: T) -> T;
    fn info(&self) -> String { "Processor trait".to_string() }
}

mod utilities {
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 { a + b }
        pub fn sub(a: i32, b: i32) -> i32 { a - b }
    }
    pub mod strings {
        pub fn capitalize(s: &str) -> String { s.to_uppercase() }
    }
}

#[derive(Debug, Clone)]
struct DataHolder<T> {
    pub values: Vec<T>,
}

impl<T: Clone> DataHolder<T> {
    fn new() -> Self { Self { values: Vec::new() } }
    fn add(&mut self, item: T) { self.values.push(item); }
    fn merge(&mut self, other: &Self) { self.values.extend_from_slice(&other.values); }
}

impl<T: Debug> Processor<T> for DataHolder<T> {
    fn process(&self, value: T) -> T { value }
}

async fn fetch_data(url: &str) -> MyResult<String> { Ok(format!("Data from {}", url)) }

enum Status { Ok, Error(String), Pending(u32) }

macro_rules! create_tuple { ($a:expr, $b:expr) => { ($a, $b) }; }

mod network {
    pub struct Request { pub endpoint: String, pub payload: Vec<u8> }

    impl Request {
        pub fn new(endpoint: &str) -> Self { Self { endpoint: endpoint.to_string(), payload: Vec::new() } }
        pub fn send(&self) -> super::Status { super::Status::Ok }
    }
}

fn calculate_sum(values: &[i32]) -> i32 { values.iter().sum() }
fn print_status(status: &Status) {
    match status {
        Status::Ok => (),
        Status::Error(_) => (),
        Status::Pending(_) => (),
    }
}

fn main() {
    let mut data = DataHolder::new();
    data.add(10);
    data.add(20);
    let sum = calculate_sum(&data.values);
    println!("Sum is {}", sum);

    let status = Status::Pending(5);
    for i in 0..3 {
        match status {
            Status::Ok => println!("Status is OK"),
            Status::Error(ref e) => println!("Error: {}", e),
            Status::Pending(val) => {
                if val > i {
                    println!("Still pending...");
                }
            }
        }
    }

    let t = create_tuple!(5, "hello");
    assert_eq!(t.0, 5);
}
// --- END test.rs ---
EOF

WALT=./target/debug/walt

echo "🔧 Encoding..."
"$WALT" encode "$TEST_FILE" "$ENCODED_FILE"

echo "🔧 Decoding..."
"$WALT" decode "$ENCODED_FILE" "$DECODED_FILE"

echo "🔍 Requiring byte-identical round trip..."
if diff -u "$TEST_FILE" "$DECODED_FILE"; then
    echo "✅ Round trip is byte-identical."
else
    echo "❌ Round trip is LOSSY: decoded output differs from the input (see diff above)."
    exit 1
fi
