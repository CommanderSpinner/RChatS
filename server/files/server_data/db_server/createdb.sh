#!/bin/bash

echo "starting db server via docker"
docker compose up -d

echo "Waiting for postgres to be ready..."
until docker exec postgres pg_isready -U admin -d postgres; do
    sleep 1
done

echo "Cleaning old db"
docker exec postgres psql -U admin -d postgres -c "DROP DATABASE IF EXISTS rchats;"
docker exec postgres psql -U admin -d postgres -c "CREATE DATABASE rchats;"

echo "Executing script to build db in postgres"
docker cp create_db.sql postgres:/create_db.sql
docker exec postgres psql -U admin -d rchats -f /create_db.sql
