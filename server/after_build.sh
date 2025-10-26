#!/bin/bash

echo "coping files and other_resources to build dir"

cp -R files ../target/debug/
cp -R other_resources ../target/debug/

# coping ocnfig for server to dir
cp config.toml ../target/debug/
cp config.toml ../target/release/

cp -R files ../target/release/
cp -R other_resources ../target/release/