#!/bin/bash

echo "coping files and other_resources to build dir"

cp -R files ../target/debug/
cp -R other_resources ../target/debug/

cp -R files ../target/release/
cp -R other_resources ../target/release/