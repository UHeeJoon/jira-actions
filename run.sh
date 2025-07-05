#!/bin/bash
set -e

echo "🔧 Building Rust binary"
cargo build --release

echo "🚀 Running CLI"
./target/release/jira-sync-action "$1" "$2"
