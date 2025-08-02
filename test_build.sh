#!/bin/bash
cd /home/des/Rust_Projects/Chimera/Echo
echo "Testing Chimera//Echo build..."
cargo check --all-targets
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "Testing movement system..."
    cargo test
    if [ $? -eq 0 ]; then
        echo "✅ Tests passed!"
    else
        echo "❌ Tests failed!"
    fi
else
    echo "❌ Build failed!"
fi