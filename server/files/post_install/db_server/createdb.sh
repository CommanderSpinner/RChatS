#!/bin/bash

echo "executing script to build db in postgres"

psql -U admin -d rchats -f create_db.sql
