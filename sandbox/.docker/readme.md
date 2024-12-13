docker network connect ghelere ghelere-nginx && docker network connect ghelere postgres-nucleos-db && docker network connect ghelere ollama && docker network connect ghelere api-ghelere-ai && docker network inspect ghelere

docker-compose exec webserver nginx -s reload

docker compose run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ -d example.org

docker compose run --rm certbot certonly --webroot --webroot-path /var/www/certbot/ --http-01-port=8080 --dry-run -d localhost

curl -I http://localhost/.well-known/acme-challenge/test-file

```
Saving debug log to /var/log/letsencrypt/letsencrypt.log
Requesting a certificate for ia.oclm.com.br

Successfully received certificate.
Certificate is saved at: /etc/letsencrypt/live/ia.oclm.com.br/fullchain.pem
Certificate is saved at: /etc/letsencrypt/live/ia.oclm.com.br/chain.pem
Key is saved at:         /etc/letsencrypt/live/ia.oclm.com.br/privkey.pem
Key is saved at:         /etc/letsencrypt/live/ia.oclm.com.br/cert.pem
This certificate expires on 2025-03-13.
These files will be updated when the certificate renews.

NEXT STEPS:
- The certificate will need to be renewed before it expires. Certbot can automatically renew the certificate in the background, but you may need to take steps to enable that functionality. See https://certbot.org/renewal-setup for instructions.

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
If you like Certbot, please consider supporting our work by:
 * Donating to ISRG / Let's Encrypt:   https://letsencrypt.org/donate
 * Donating to EFF:                    https://eff.org/donate-le
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
```