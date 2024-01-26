#!/bin/bash

cargo build --release
cp target/release/zat ~/bin/
zat -V
