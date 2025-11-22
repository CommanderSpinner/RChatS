#!/bin/bash

docker compose up -d

echo "ckeaning old db"

docker exec -it postgres psql -U admin -d postgres -c "DROP DATABASE IF EXISTS rchats; CREATE DATABASE rchats;"

echo "executing script to build db in postgres"

docker exec -it postgres psql -U admin -d rchats -f create_db.sql
