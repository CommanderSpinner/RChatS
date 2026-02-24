#!/bin/bash

echo "starting db server via docker"
docker compose up -d

echo "Waiting for postgres to be ready..."
until docker exec postgres pg_isready -U admin -d postgres; do
    sleep 1
done

if [[ "$1" = "clean" ]]; then
    echo "Cleaning old db"
    echo "Executing script to build db in postgres"
    docker cp create_db.sql postgres:/create_db.sql
    docker exec postgres psql -U admin -d rchats -f /create_db.sql
fi


