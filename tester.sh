#!/usr/bin/env bash
set -euo pipefail

TMP_DIR=$(mktemp -d)
TEST_FILE="$TMP_DIR/test.rs"
ENCODED_FILE="$TMP_DIR/encoded.txt"
DECODED_FILE="$TMP_DIR/decoded_test.rs"

echo "📝 Creating complex test Rust source file..."
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

echo "🔧 Running encoder..."
if ! walt encode "$TEST_FILE" "$ENCODED_FILE"; then
    echo "❌ Encoder failed"
    exit 1
fi

# Ensure encoded file exists
if [ ! -f "$ENCODED_FILE" ]; then
    echo "❌ Encoded file not found!"
    exit 1
fi

echo "🔧 Running decoder..."
if ! walt decode "$ENCODED_FILE" "$DECODED_FILE"; then
    echo "❌ Decoder failed"
    exit 1
fi

# Ensure decoded file exists
if [ ! -f "$DECODED_FILE" ]; then
    echo "❌ Decoded file not found!"
    exit 1
fi

echo "🔍 Comparing syntax element accuracy..."

declare -A ELEMENTS=(
    ["traits"]="trait Processor"
    ["use_statements"]="use std::collections"
    ["impl_blocks"]="impl<T: Debug> Processor<T>"
    ["functions"]="fn calculate_sum"
    ["statics"]="static mut GLOBAL_STATE"
    ["constants"]="const DEFAULT_CAPACITY"
    ["macros"]="macro_rules! create_tuple"
    ["type_aliases"]="type MyResult"
    ["structs"]="struct DataHolder"
    ["enums"]="enum Status"
)

compute_accuracy() {
    local orig="$1"
    local dec="$2"
    local total_chars=${#orig}
    local matched_chars=0

    for ((i=0; i<total_chars; i++)); do
        [[ "${orig:i:1}" == "${dec:i:1}" ]] && ((matched_chars++))
    done

    if (( total_chars == 0 )); then
        echo "100.00"
    else
        awk "BEGIN {printf \"%.2f\", ($matched_chars/$total_chars)*100}"
    fi
}

for key in "${!ELEMENTS[@]}"; do
    pattern="${ELEMENTS[$key]}"
    orig_match=$(grep -E "$pattern" "$TEST_FILE" || true)
    dec_match=$(grep -E "$pattern" "$DECODED_FILE" || true)

    if [ -z "$dec_match" ]; then
        echo "❌ $key FAILED: missing or mismatched"
    else
        accuracy=$(compute_accuracy "$orig_match" "$dec_match")
        echo "✅ $key | Accuracy: $accuracy%"
    fi
done

echo "🎉 Test complete! Decoded file at: $DECODED_FILE"
