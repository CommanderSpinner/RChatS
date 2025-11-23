#!/bin/bash

set -e

echo "cleaning"
cargo clean

echo "building"
cargo build

echo "coping files for post install and server to build dir"

cp -R files ../target/debug/

echo "executing script start db server"

bash ../target/debug/files/server_data/db_server/createdb.sh

echo "testing server"

../target/debug/server