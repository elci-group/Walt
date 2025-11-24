# Walt VHS Demo Tape
# Run `vhs demo.tape` to generate `demo.gif`

Output "demo.gif"

Set Theme "Dracula"
Set Width 1200
Set Height 800
Set TypingSpeed 100ms

# Show the initial source code
Type "cat demo_src.rs"
Enter
Sleep 2s

# Build the project first to ensure it's up to date
Type "cargo build --quiet"
Enter
Sleep 3s

# Encode the file
Type "walt encode demo_src.rs demo.ars"
Enter
Sleep 2s

# Show the encoded .ars file (it will look like RON)
Type "cat demo.ars"
Enter
Sleep 3s
Hide

# Decode the file
Type "walt decode demo.ars decoded.rs"
Enter
Sleep 2s

# Show the reconstructed file and compare with the original
Type "diff --side-by-side demo_src.rs decoded.rs"
Enter
Sleep 3s

# Clean up and finish
Type "rm demo_src.rs demo.ars decoded.rs"
Enter
Sleep 2s
