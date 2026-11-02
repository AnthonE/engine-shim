#!/bin/sh
# Play: Facepunch's Sandbox hosted on this library, and you joining it in a
# window, on this library too. Run from the repository's root after
# tools/setup.sh, on a machine with a display (X11 or Wayland) and a GPU.
#   sh tools/play.sh [zoo|flatgrass] [width height]
# W A S D to walk, the mouse to look. Close the window to stop both.
# Logs: target/spike/server.log, target/play.log.
set -e
ROOT=$(pwd)
SBOX=$ROOT/../sbox-public
SANDBOX=$ROOT/../sandbox
where=${1:-zoo}; width=${2:-1280}; height=${3:-720}
[ -z "$DOTNET_ROOT" ] && [ ! -x "$HOME/.dotnet/dotnet" ] && command -v dotnet >/dev/null && DOTNET_ROOT=$(dirname "$(readlink -f "$(command -v dotnet)")")
export DOTNET_ROOT=${DOTNET_ROOT:-$HOME/.dotnet} PATH=$HOME/.dotnet:$PATH

[ -d "$SANDBOX/.git" ] || git clone -q https://github.com/Facepunch/sandbox "$SANDBOX"
sh tools/compile.sh "$SBOX/game/core" surface prefab sound decal
content="$SANDBOX/Assets"
if [ "$where" = flatgrass ]; then
	sh tools/map.sh facepunch.flatgrass "$SANDBOX/Assets"
	sh tools/package.sh facepunch.flatgrass "$ROOT/../content-flatgrass" '\.(vmat_c|vtex_c|vmdl_c|vmesh_c)$'
	sh tools/sandbox.sh "$SANDBOX" scenes/sandbox.scene
	content="$content:$ROOT/../content-flatgrass"
else
	sh tools/sandbox.sh "$SANDBOX"
	# the zoo's editor meshes are drawn in a dev material flatgrass's package has
	[ -d "$ROOT/../content-flatgrass/materials/dev" ] || sh tools/package.sh facepunch.flatgrass "$ROOT/../content-flatgrass" '^materials/dev/'
	content="$content:$ROOT/../content-flatgrass"
fi

mkdir -p target/spike
rm -f target/spike/server.log
SBOX=$SBOX sh spike.sh 86400 +shimgame "$SANDBOX/sandbox.sbproj" +shimhost &
server=$!
trap 'kill $server 2>/dev/null; pkill -f "sbox-server +shimgame" 2>/dev/null || true' EXIT
until grep -q "Server started!" target/spike/server.log 2>/dev/null; do
	kill -0 "$server" 2>/dev/null || { echo "the server stopped before it started"; tail -20 target/spike/server.log; exit 1; }
	sleep 2
done

echo "joining, in a window ${width}x${height}"
(cd "$SBOX/game" && ENGINE2_CONTENT="$content" ./sbox-server +shimclient 127.0.0.1:27015 +shimwindow -width "$width" -height "$height" +instanceid 2 > "$ROOT/target/play.log" 2>&1) || true
