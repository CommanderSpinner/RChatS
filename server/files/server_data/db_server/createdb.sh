#!/bin/bash


if [[ "$1" = "clean" ]]; then
    docker compose down --volumes --remove-orphans
fi

echo "starting db server via docker"
docker compose up -d