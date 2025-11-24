#! /bin/bash

echo "Here is the original main.rs"
cat ~/walt_v1/src/main.rs
echo "Encoding with walt" 
cargo run -- encode ~/walt_v1/src/main.rs ~/walt_v1/src/test_A.ars || (echo "Encoding failed" & exit)

echo "Here is the .ars version of the main.rs script"
cat ~/walt_v1/src/test_A.ars
echo "Let's try decoding the .ars back to .rs"
cargo run -- decode ~/walt_v1/src/test_A.ars ~/walt_v1/src/test_B.rs || echo "Decoding failed"
echo "Here is decoded version"
cat ~/walt_v1/src/test_B.rs


