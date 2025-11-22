#!/bin/bash

docker compose up -d

echo "Cleaning old db"
docker exec -it postgres psql -U admin -d postgres -c "DROP DATABASE IF EXISTS rchats;"
docker exec -it postgres psql -U admin -d postgres -c "CREATE DATABASE rchats;"

echo "Executing script to build db in postgres"
docker cp create_db.sql postgres:/create_db.sql
docker exec -it postgres psql -U admin -d rchats -f /create_db.sql

