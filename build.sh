#!/bin/bash
# Build and test script for Py2Rust

set -e

echo "🔨 Building Py2Rust..."
cargo build --release

echo "✅ Build successful!"

echo ""
echo "🧪 Running tests..."
cargo test --release -- --nocapture

echo ""
echo "📚 Generating documentation..."
cargo doc --no-deps --release

echo ""
echo "✨ All checks passed!"
