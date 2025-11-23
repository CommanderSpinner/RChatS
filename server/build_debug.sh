#!/bin/bash

set -e

if [[ "$1" = "clean" ]]; then
    echo "cleaning"
    cargo clean
fi

echo "building"
cargo build

echo "coping files for post install and server to build dir"
cp -R files ../target/debug/

echo "executing script start db server"
cd ../target/debug/files/server_data/db_server/
pwd

if [[ "$1" = "clean" ]]; then
    bash createdb.sh clean
else
    bash createdb.sh
fi

echo "testing server"
cd ../../../
pwd
./server