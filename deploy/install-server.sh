#!/bin/sh
# On the game box, as root: a game's server from a tools/build.sh folder (or
# the linux-x86_64 depot tools/publish.sh staged: the same files), installed
# as shim-server@<slug> and started. Run again with a new build to update it
# (its settings, key and shard row are kept).
#   sh deploy/install-server.sh <slug> <build dir> <host> [port] [status port]
#   sh deploy/install-server.sh sandbox /tmp/sandbox-build game.elopros.com 27015 27016
# Then: the certificate (deploy/letsencrypt-hook.sh, once by hand, then as a
# renewal hook), nginx's two locations (deploy/nginx-status.conf), UDP <port>
# open in ufw and in the cloud firewall in front of it, and the shard row
# copied to the origin (the last lines this prints).
set -e
here=$(cd "$(dirname "$0")" && pwd)
slug=${1:?a slug}; build=$(cd "${2:?a build folder}" && pwd); host=${3:?the name players reach it at}
port=${4:-27015}; sport=${5:-$((port + 1))}
[ -x "$build/host" ] || { echo "install: $build has no host (a linux-x86_64 build from tools/build.sh)" >&2; exit 2; }
id elo-shim >/dev/null 2>&1 || useradd --system --home-dir /var/lib/elo-shim --shell /usr/sbin/nologin elo-shim
mkdir -p /opt/elo-shim /var/lib/elo-shim/$slug /etc/elo-shim/tls
chown elo-shim:elo-shim /var/lib/elo-shim/$slug
# the build, swapped in whole; what the engine wrote (data, logs, packages it
# downloaded) carried over
new=/opt/elo-shim/.$slug.new
rm -rf "$new"; cp -a "$build" "$new"
for keep in data logs download; do
	[ -d /opt/elo-shim/$slug/game/$keep ] && mv /opt/elo-shim/$slug/game/$keep "$new/game/$keep"
done
# a depot names files, not links: the module names are copies there, links here
nat=$new/game/bin/linuxsteamrt64
for m in vfx_vulkan filesystem_stdio; do ln -sf libengine2.so "$nat/lib$m.so"; done
chmod +x "$new/host" "$new/play" "$new/dotnet/dotnet"
chown -R elo-shim:elo-shim "$new/game"
rm -rf /opt/elo-shim/$slug.old; [ -d /opt/elo-shim/$slug ] && mv /opt/elo-shim/$slug /opt/elo-shim/$slug.old
mv "$new" /opt/elo-shim/$slug
# its settings, once (kept on update)
if [ ! -f /etc/elo-shim/$slug.env ]; then
	sed -e "s/<slug>/$slug/g" -e "s/<host>/$host/g" -e "s/<port>/$port/g" -e "s/<status port>/$sport/g" \
		"$here/shim-server.env" > /etc/elo-shim/$slug.env
fi
install -m 0644 "$here/shim-server@.service" /etc/systemd/system/shim-server@.service
systemctl daemon-reload
systemctl enable shim-server@$slug >/dev/null 2>&1
systemctl restart shim-server@$slug
rm -rf /opt/elo-shim/$slug.old
echo "shim-server@$slug: started (journalctl -u shim-server@$slug -f)"
[ -s /etc/elo-shim/tls/fullchain.pem ] || echo "  no certificate yet: CERT_NAME=$host sh $here/letsencrypt-hook.sh (after certbot has one for $host)"
echo "  nginx, in $host's https server block:"
sed -e "s/<slug>/$slug/g" -e "s/<status port>/$sport/g" "$here/nginx-status.conf" | grep '^location' | sed 's/^/    /'
echo "  ufw allow $port/udp, and the same in the cloud firewall"
echo "  once it says 'Server started!', its row to the origin:"
echo "    rsync -a /var/lib/elo-shim/$slug/servers.json <origin>:\$SCRY_DEPOTS_DIR/$slug/servers.json"
