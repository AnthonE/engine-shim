#!/bin/sh
# /etc/letsencrypt/renewal-hooks/deploy/elo-shim.sh: the game servers'
# certificate copied where their user can read it, and the servers
# restarted, on every renewal (as the relay's hook does).
#   CERT_NAME=game.elopros.com (the name the servers' addr uses)
set -e
name=${CERT_NAME:-game.elopros.com}
live=/etc/letsencrypt/live/$name
mkdir -p /etc/elo-shim/tls
install -m 0644 -o elo-shim -g elo-shim "$live/fullchain.pem" /etc/elo-shim/tls/fullchain.pem
install -m 0600 -o elo-shim -g elo-shim "$live/privkey.pem" /etc/elo-shim/tls/privkey.pem
systemctl try-restart 'shim-server@*.service'
