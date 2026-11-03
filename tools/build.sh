#!/bin/sh
# A game as a player installs it: one folder that runs with nothing else on
# the machine. No .NET SDK, no sbox-public checkout, no Rust. In it, the .NET
# runtime (Microsoft's, MIT), s&box's managed engine as tools/setup.sh built
# it (MIT; none of game/bin's closed natives), this repository's
# libengine2.so, the game project, and what it was told to bring from
# sbox.game (under its creator's terms: a build you ship carries only content
# you may ship). The packages a game names on sbox.game (Sandbox's props and
# weapons) are what a player still downloads: the server lists them when
# they join and they install into game/download/ from sbox.game, as s&box's
# own client does, kept for the next time.
#   sh tools/build.sh <project dir> <out dir> [--platform linux-x86_64|win-x86_64] [--map org.ident]... [--package org.ident[=regex]]...
#   sh tools/build.sh ../sandbox ../sandbox-build --package 'facepunch.flatgrass=^materials/dev/'
# Then, from anywhere (copy the folder where you like):
# (win-x86_64 cross-compiles from Linux: mingw-w64 and rustup's
# x86_64-pc-windows-gnu; its folder has host.cmd and play.cmd.)
#   <out>/host                     the game's dedicated server, on 27015
#   <out>/play [host:port|best]    join one, in a window
# Run from the repository's root after tools/setup.sh. A project from
# tools/sandbox.sh wants SHIMWATCH=0 (the test watcher left out).
set -e
ROOT=$(pwd)
SBOX=${SBOX:-$ROOT/../sbox-public}
proj=$(cd "${1:?a game project directory (with its .sbproj)}" && pwd)
out=${2:?an output directory}; shift 2
maps=; packages=; platform=linux-x86_64
while [ $# -gt 0 ]; do
	case $1 in
		--platform) platform=$2; shift 2 ;;
		--map) maps="$maps $2"; shift 2 ;;
		--package) packages="$packages $2"; shift 2 ;;
		*) echo "build: unknown argument $1" >&2; exit 2 ;;
	esac
done
case $platform in linux-x86_64|win-x86_64) ;; *) echo "build: no platform $platform (linux-x86_64, win-x86_64)" >&2; exit 2 ;; esac
sbproj=$(ls "$proj"/*.sbproj 2>/dev/null | head -n 1)
[ -n "$sbproj" ] || { echo "build: no .sbproj in $proj" >&2; exit 2; }
[ -x "$SBOX/game/sbox-server" ] || { echo "build: $SBOX/game has no sbox-server; run tools/setup.sh" >&2; exit 2; }
[ -f "$proj/Code/ShimWatch.cs" ] && { echo "build: $proj has the test watcher (Code/ShimWatch.cs); rerun tools/sandbox.sh with SHIMWATCH=0" >&2; exit 2; }
[ -z "$DOTNET_ROOT" ] && [ ! -x "$HOME/.dotnet/dotnet" ] && command -v dotnet >/dev/null && DOTNET_ROOT=$(dirname "$(readlink -f "$(command -v dotnet)")")
export DOTNET_ROOT=${DOTNET_ROOT:-$HOME/.dotnet} PATH=$HOME/.dotnet:$PATH

# the engine library, and core's resources as it reads them
dotnet run --project gen -- "$SBOX" engine engine2/src/slots.rs
if [ "$platform" = win-x86_64 ]; then
	cargo build --release -q -p engine2 --target x86_64-pc-windows-gnu
else
	cargo build --release -q -p engine2
fi
sh tools/compile.sh "$SBOX/game/core" surface prefab sound decal

rm -rf "$out"
mkdir -p "$out/game" "$out/dotnet" "$out/project" "$out/content"
out=$(cd "$out" && pwd)

# s&box's game folder: the managed engine and its content, without the editor,
# the samples, the closed natives (bin/win64) or the API docs; and without the
# other platform's natives and the Linux apphost
(cd "$SBOX/game" && tar cf - \
	--exclude=./bin/win64 --exclude=./bin/linuxsteamrt64 --exclude=./sbox-server \
	--exclude=./editor --exclude=./samples --exclude=./templates \
	--exclude=./unittest --exclude=./addons/editor --exclude=./addons/tools \
	--exclude='./bin/managed/*.xml' --exclude='./bin/managed/*.so' \
	--exclude=./data --exclude=./logs --exclude=./download --exclude='./.source2' .) | (cd "$out/game" && tar xf -)

fx=$(sed -n 's/.*"name": *"\(Microsoft\.[A-Za-z.]*\)".*/\1/p' "$SBOX/game/sbox-server.runtimeconfig.json")
rv=$(ls "$DOTNET_ROOT/shared/$fx" | sort -V | tail -n 1)
if [ "$platform" = win-x86_64 ]; then
	# what's fetched for Windows, once (target/win-x64/)
	cache=$ROOT/target/win-x64; mkdir -p "$cache"
	fetch() { [ -s "$cache/$2" ] || curl -sSfL "$1" -o "$cache/$2"; }
	nat=$out/game/bin/win64; mkdir -p "$nat"
	x86_64-w64-mingw32-strip --strip-debug -o "$nat/engine2.dll" target/x86_64-pc-windows-gnu/release/engine2.dll
	# the engine modules the managed side loads by name, all this one library
	for m in vfx_vulkan filesystem_stdio; do cp "$nat/engine2.dll" "$nat/$m.dll"; done
	# SkiaSharp's and HarfBuzz's Windows natives (MIT, NuGet), as setup does Linux's
	for pkg in skiasharp.nativeassets.win32:3.116.0 harfbuzzsharp.nativeassets.win32:8.3.0; do
		id=${pkg%%:*}; v=${pkg#*:}
		fetch "https://api.nuget.org/v3-flatcontainer/$id/$v/$id.$v.nupkg" "$id.$v.zip"
		unzip -q -o -j "$cache/$id.$v.zip" 'runtimes/win-x64/native/*.dll' -d "$nat"
	done
	cp "$nat"/libSkiaSharp.dll "$nat"/libHarfBuzzSharp.dll "$out/game/bin/managed/"
	# Microsoft's .NET runtime for Windows, the version this machine's is
	fetch "https://builds.dotnet.microsoft.com/dotnet/Runtime/$rv/dotnet-runtime-$rv-win-x64.zip" "dotnet-runtime-$rv-win-x64.zip"
	unzip -q -o "$cache/dotnet-runtime-$rv-win-x64.zip" -d "$out/dotnet"
	hv=$(ls "$out/dotnet/host/fxr" | sort -V | tail -n 1)
else
	nat=$out/game/bin/linuxsteamrt64; mkdir -p "$nat"
	cp "$SBOX"/game/bin/linuxsteamrt64/libSkiaSharp.so "$SBOX"/game/bin/linuxsteamrt64/libHarfBuzzSharp.so "$nat/"
	cp "$nat"/libSkiaSharp.so "$nat"/libHarfBuzzSharp.so "$out/game/bin/managed/"
	# without its debug info (the release profile keeps it for profiling: 180 MB)
	strip --strip-debug -o "$nat/libengine2.so" target/release/libengine2.so
	# the engine modules the managed side loads by name, all this one library
	for m in vfx_vulkan filesystem_stdio; do ln -sf libengine2.so "$nat/lib$m.so"; done
	# the .NET runtime the server's runtimeconfig names (the shared framework
	# and the host that finds it), not the SDK
	hv=$(ls "$DOTNET_ROOT/host/fxr" | sort -V | tail -n 1)
	mkdir -p "$out/dotnet/shared/$fx" "$out/dotnet/host/fxr"
	cp "$DOTNET_ROOT/dotnet" "$out/dotnet/"
	cp -r "$DOTNET_ROOT/shared/$fx/$rv" "$out/dotnet/shared/$fx/"
	cp -r "$DOTNET_ROOT/host/fxr/$hv" "$out/dotnet/host/fxr/"
	cp "$DOTNET_ROOT/LICENSE.txt" "$DOTNET_ROOT/ThirdPartyNotices.txt" "$out/dotnet/" 2>/dev/null || true
fi

# the game: its project (the server compiles its code; a player is sent it),
# its maps' paks, and the packages' files a player reads from disk
name=$(basename "$sbproj" .sbproj)
(cd "$proj" && tar cf - --exclude=./.git --exclude=./.sbox --exclude=./obj --exclude=./bin .) | (cd "$out/project" && tar xf -)
for m in $maps; do sh tools/map.sh "$m" "$out/project/Assets"; done
for p in $packages; do
	ident=${p%%=*}; pattern=.; [ "$ident" != "$p" ] && pattern=${p#*=}
	sh tools/package.sh "$ident" "$out/content" "$pattern"
done

if [ "$platform" = linux-x86_64 ]; then
cat > "$out/host" <<EOF
#!/bin/sh
# This game's dedicated server, on this folder alone. Its arguments go to the
# server (s&box's own, and engine-shim's ENGINE2_* environment: README).
here=\$(cd "\$(dirname "\$0")" && pwd)
export DOTNET_ROOT="\$here/dotnet"
cd "\$here/game"
exec "\$here/dotnet/dotnet" sbox-server.dll +shimgame "\$here/project/$(basename "$sbproj")" +shimhost "\$@"
EOF
cat > "$out/play" <<'EOF'
#!/bin/sh
# Join a server in a window, on this folder alone:
#   ./play [host:port | identity | best] [--servers <list>] [engine arguments...]
# best (or nothing) joins the busiest server on the list that has room. The
# list is a shard list's URL or file (ENGINE2_SERVER_LIST): the Elo Pros
# launcher passes its title's. -width and -height go to the engine
# (1280x720 unless given).
here=$(cd "$(dirname "$0")" && pwd)
export DOTNET_ROOT="$here/dotnet"
export ENGINE2_CONTENT="$here/project/Assets:$here/content${ENGINE2_CONTENT:+:$ENGINE2_CONTENT}"
address=${1:-best}; [ $# -gt 0 ] && shift
if [ "$1" = --servers ]; then
	[ -n "$2" ] && export ENGINE2_SERVER_LIST="$2"
	shift; [ $# -gt 0 ] && shift
fi
size="-width 1280 -height 720"
case " $* " in *" -width "*|*" -height "*) size= ;; esac
cd "$here/game"
exec "$here/dotnet/dotnet" sbox-server.dll +shimclient "$address" +shimwindow $size +instanceid 2 "$@"
EOF
chmod +x "$out/host" "$out/play"
else
	# Windows: cmd scripts, CRLF; the folder's own dotnet.exe finds its runtime
	# beside itself
	sbname=$(basename "$sbproj")
	{
		echo '@echo off'
		echo 'rem This game'"'"'s dedicated server, on this folder alone. Its arguments go to the server.'
		echo 'setlocal'
		echo 'set "HERE=%~dp0"'
		echo 'cd /d "%HERE%game"'
		echo '"%HERE%dotnet\dotnet.exe" sbox-server.dll +shimgame "%HERE%project\'"$sbname"'" +shimhost %*'
	} | sed 's/$/\r/' > "$out/host.cmd"
	{
		echo '@echo off'
		echo 'rem Join a server in a window, on this folder alone: play.cmd [host:port ^| identity ^| best]'
		echo 'rem best (or nothing) joins the busiest server on ENGINE2_SERVER_LIST that has room.'
		echo 'setlocal'
		echo 'set "HERE=%~dp0"'
		echo 'set "ENGINE2_CONTENT=%HERE%project\Assets;%HERE%content;%ENGINE2_CONTENT%"'
		echo 'set "ADDR=%~1"'
		echo 'if "%ADDR%"=="" set "ADDR=best"'
		echo 'cd /d "%HERE%game"'
		echo '"%HERE%dotnet\dotnet.exe" sbox-server.dll +shimclient %ADDR% +shimwindow -width 1280 -height 720 +instanceid 2'
	} | sed 's/$/\r/' > "$out/play.cmd"
fi

rev() { git -C "$1" rev-parse --short HEAD 2>/dev/null || echo none; }
cat > "$out/BUILD.txt" <<EOF
$name, built $(date -u +%Y-%m-%dT%H:%M:%SZ)
engine-shim $(rev "$ROOT")$(git -C "$ROOT" diff --quiet HEAD 2>/dev/null || echo ' (with uncommitted changes)')
sbox-public $(rev "$SBOX") (and sbox-public.patch)
$name $(rev "$proj")
.NET $fx $rv, host $hv, for $platform
maps:$maps
packages:$packages

./host (host.cmd)                  the dedicated server
./play (play.cmd) [host:port|best]  join one in a window (best: the busiest on the list)

On Linux a machine wants, besides: ALSA (libasound2), a Vulkan driver
(libvulkan1 and your GPU's, or mesa-vulkan-drivers), and for the window
libxkbcommon-x11-0 with X11 or Wayland. Windows 10 or later has what it needs.
EOF
echo "built $name: $out ($(du -sh "$out" | cut -f1))"
