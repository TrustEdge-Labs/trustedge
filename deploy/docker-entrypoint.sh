#!/bin/sh
set -e

# Conditionally activate HTTPS server block when cert paths are configured.
# Set SSL_CERT_PATH and SSL_KEY_PATH env vars to enable TLS termination.
if [ -n "${SSL_CERT_PATH}" ] && [ -n "${SSL_KEY_PATH}" ]; then
    envsubst '${SSL_CERT_PATH} ${SSL_KEY_PATH}' \
        < /etc/nginx/conf.d/nginx-ssl.conf.template \
        > /etc/nginx/conf.d/ssl.conf
fi

exec nginx -g "daemon off;"
