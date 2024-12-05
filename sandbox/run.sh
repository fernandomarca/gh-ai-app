#!/bin/bash

# Criar as docker networks
docker network create ai-dev-services

docker compose -f docker-compose.yaml up -d

echo "Inicializando os containers..."
sleep 20