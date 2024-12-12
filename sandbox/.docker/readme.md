docker network connect ghelere ghelere-nginx && docker network connect ghelere postgres-nucleos-db && docker network connect ghelere ollama && docker network connect ghelere api-ghelere-ai && docker network inspect ghelere

docker-compose exec webserver nginx -s reload

docker compose run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ -d example.org

