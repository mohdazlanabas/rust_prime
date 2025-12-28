#!/bin/bash
# run.sh

echo "🚀 Building and launching Prime Hunter..."
cargo build --release && ./target/release/prime_hunter