#!/bin/bash
# inject.sh - append generic extract/reconstruct stubs to Rust modules

MODULE_DIR="src/syntax_elements"

# Generic stub
read -r -d '' STUB <<'EOF'

// Generic extract function stub
pub fn extract(source: &str) -> Vec<String> {
    source
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

// Generic reconstruct function stub
pub fn reconstruct(lines: &[String]) -> String {
    lines.join("\n")
}
EOF

# Check if directory exists
if [ ! -d "$MODULE_DIR" ]; then
    echo "Error: $MODULE_DIR does not exist!"
    exit 1
fi

# Loop through all .rs files
shopt -s nullglob  # ensures *.rs expands to empty array if no matches
files=("$MODULE_DIR"/*.rs)

if [ ${#files[@]} -eq 0 ]; then
    echo "No .rs files found in $MODULE_DIR"
    exit 0
fi

for file in "${files[@]}"; do
    echo "Processing $file ..."

    if grep -q "fn extract" "$file" && grep -q "fn reconstruct" "$file"; then
        echo "  -> Already has extract & reconstruct. Skipping."
    else
        echo "$STUB" >> "$file"
        echo "  -> Appended generic stub."
    fi
done

echo "Done."
