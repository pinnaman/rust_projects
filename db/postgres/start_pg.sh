#!/bin/bash

docker-compose down

sleep 10

#docker-compose -f docker-compose-postgres.yml up -d
docker-compose  up -d
