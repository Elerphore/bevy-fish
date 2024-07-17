#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/bevy-fish.exe . &&
exec ./bevy-fish.exe "$@"

