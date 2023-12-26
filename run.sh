#!/bin/sh
RUST_BACKTRACE=1 cargo test --bin $1 --release
if [ $? -eq 0 ]; then
    RUST_BACKTRACE=1 cargo run --bin $1 --release ./input/$1
fi
