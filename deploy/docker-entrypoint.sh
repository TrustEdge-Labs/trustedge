#!/bin/sh
set -e

# Conditionally activate HTTPS server block when cert paths are configured.
# Set SSL_CERT_PATH and SSL_KEY_PATH env vars to enable TLS termination.
#
# Output goes to /tmp/nginx-ssl/ (writable by non-root uid 101).
# The static include directive in /etc/nginx/conf.d/ssl-include.conf picks it up.
if [ -n "${SSL_CERT_PATH}" ] && [ -n "${SSL_KEY_PATH}" ]; then
    envsubst '${SSL_CERT_PATH} ${SSL_KEY_PATH}' \
        < /etc/nginx/nginx-ssl.conf.template \
        > /tmp/nginx-ssl/ssl.conf
fi

exec nginx -g "daemon off;"
