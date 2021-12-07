#!/bin/sh
rm -f wave.txt
cargo run > wave.txt
../plot.py wave.txt

