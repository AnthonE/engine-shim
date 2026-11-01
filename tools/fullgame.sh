#!/bin/sh
# The whole game, end to end, as the README's Sandbox steps: Facepunch's
# Sandbox (github.com/Facepunch/sandbox, MIT) cloned beside this checkout,
# core's resources compiled, flatgrass's paks and materials fetched from
# sbox.game (never committed), its dedicated server hosting it on this
# library, and a headless player joining it, whose watcher
# (tools/sandbox/ShimWatch.cs) asks the scene's camera for a frame every ten
# seconds. Run from the repository's root after tools/setup.sh.
#   sh tools/fullgame.sh [seconds]
# Logs: target/spike/server.log, target/spike/client.log; the frames' paths
# are in the client's "shimwatch view" lines.
set -e
ROOT=$(pwd)
SBOX=$ROOT/../sbox-public
SANDBOX=$ROOT/../sandbox
CONTENT=$ROOT/../content-flatgrass
secs=${1:-300}
# the SDK: ours in ~/.dotnet, else the system's (a distribution's dotnet-sdk-10.0)
[ -z "$DOTNET_ROOT" ] && [ ! -x "$HOME/.dotnet/dotnet" ] && command -v dotnet >/dev/null && DOTNET_ROOT=$(dirname "$(readlink -f "$(command -v dotnet)")")
export DOTNET_ROOT=${DOTNET_ROOT:-$HOME/.dotnet} PATH=$HOME/.dotnet:$PATH

[ -d "$SANDBOX/.git" ] || git clone -q https://github.com/Facepunch/sandbox "$SANDBOX"
sh tools/compile.sh "$SBOX/game/core" surface prefab sound decal
sh tools/map.sh facepunch.flatgrass "$SANDBOX/Assets"
sh tools/package.sh facepunch.flatgrass "$CONTENT" '\.(vmat_c|vtex_c|vmdl_c|vmesh_c)$'
sh tools/sandbox.sh "$SANDBOX" scenes/sandbox.scene

# the server, hosting it
mkdir -p target/spike
rm -f target/spike/server.log target/spike/client.log
SBOX=$SBOX sh spike.sh "$secs" +shimgame "$SANDBOX/sandbox.sbproj" +shimhost &
server=$!
until grep -q "Server started!" target/spike/server.log 2>/dev/null; do
	kill -0 "$server" 2>/dev/null || { echo "the server stopped before it started"; tail -20 target/spike/server.log; exit 1; }
	sleep 5
done

# a player, its copy of the map and its materials where it looks for them
(cd "$SBOX/game" && ENGINE2_CONTENT="$SANDBOX/Assets:$CONTENT" timeout $((secs - 60)) ./sbox-server +shimclient 127.0.0.1:27015 +instanceid 2 > "$ROOT/target/spike/client.log" 2>&1) || true
wait "$server" || true

echo "-- server"
grep shimwatch target/spike/server.log | tail -20 || true
echo "-- player"
grep "shimwatch\|render:" target/spike/client.log | tail -20 || true
