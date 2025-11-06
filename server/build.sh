#!/bin/bash

set -e

echo "cleaning"
cargo clean

echo "building"
cargo build

echo "coping files for post install and server to build dir"

cp -R files ../target/debug/
cp -R files ../target/release/

echo "testing server"

../target/debug/server