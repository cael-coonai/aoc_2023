#!/bin/sh
RUST_BACKTRACE=1 cargo run --bin $1 --release ./input/$1
