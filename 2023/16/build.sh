#!/bin/fish
RUSTFLAGS='-C target-feature=+crt-static -C target-cpu=native' cargo build -r 