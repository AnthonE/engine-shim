#!/bin/sh
# A fresh machine made ready for spike.sh: sbox-public at our commit, patched,
# its interop generated, the managed dedicated server built, and the MIT
# natives and reference assemblies it needs beside it. Run from the
# repository's root; the checkout goes beside it (../sbox-public).
#   sh tools/setup.sh [sbox-public dir]
set -e
ROOT=$(pwd)
SBOX=${1:-$ROOT/../sbox-public}
# the SDK: ours in ~/.dotnet, else the system's (a distribution's dotnet-sdk-10.0)
[ -z "$DOTNET_ROOT" ] && [ ! -x "$HOME/.dotnet/dotnet" ] && command -v dotnet >/dev/null && DOTNET_ROOT=$(dirname "$(readlink -f "$(command -v dotnet)")")
export DOTNET_ROOT=${DOTNET_ROOT:-$HOME/.dotnet} PATH=$HOME/.dotnet:$PATH
SKIA=3.116.0

# the client's sound card wants ALSA's headers to build against (Linux)
if [ "$(uname)" = Linux ] && ! pkg-config --exists alsa 2>/dev/null; then
	echo "setup: ALSA's headers are missing; install them (apt-get install libasound2-dev pkg-config) or build with --no-default-features (a server)"
fi
# a client's window and frames, at run time: winit loads xkbcommon's X11 half
# for the keyboard (without it the window's thread panics), wgpu a Vulkan
# driver (lavapipe where there's no GPU)
if [ "$(uname)" = Linux ]; then
	ldconfig -p 2>/dev/null | grep -q 'libxkbcommon-x11\.so' || echo "setup: a windowed client wants libxkbcommon-x11 (apt-get install libxkbcommon-x11-0)"
	ldconfig -p 2>/dev/null | grep -q 'libvulkan\.so\.1' || echo "setup: a client draws through Vulkan (apt-get install mesa-vulkan-drivers: lavapipe, where there's no GPU)"
fi

if ! command -v dotnet >/dev/null; then
	curl -sSL https://dot.net/v1/dotnet-install.sh -o /tmp/dotnet-install.sh
	bash /tmp/dotnet-install.sh --channel 10.0 --install-dir "$HOME/.dotnet"
fi

if [ ! -d "$SBOX/.git" ]; then
	git clone --filter=blob:none https://github.com/Facepunch/sbox-public "$SBOX"
fi
cd "$SBOX"
git checkout -q "$(cat "$ROOT/SBOX_PUBLIC_COMMIT")"
git apply --check "$ROOT/sbox-public.patch" 2>/dev/null && git apply "$ROOT/sbox-public.patch"

# the interop: Facepunch's generator, run once, into folders that must exist
if [ ! -f engine/Sandbox.Engine/Interop.Engine.cs ]; then
	for def in $(find engine/Definitions -name '*.def'); do
		grep -E '^\s*(cpp|hpp|output)\s' "$def" | sed 's/.*"\(.*\)".*/\1/' | while read -r p; do
			mkdir -p "$(dirname "$def")/$(dirname "$p")" "engine/$(dirname "$p")" "$(dirname "$p")"
		done
	done
	g=$(mktemp -d)
	cat > "$g/igen.csproj" <<EOF
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework></PropertyGroup>
  <ItemGroup><ProjectReference Include="$SBOX/engine/Tools/InteropGen/InteropGen.csproj" /></ItemGroup>
</Project>
EOF
	echo 'Facepunch.InteropGen.Program.ProcessManifest("engine", false);' > "$g/Program.cs"
	dotnet build -c Release -v q "$g/igen.csproj" -o "$g/out"
	dotnet "$g/out/igen.dll"
	rm -rf "$g"
fi

# .NET's reference assemblies, which the engine's C# compiler embeds
ref=$(ls -d "$DOTNET_ROOT"/packs/Microsoft.NETCore.App.Ref/10.*/ref/net10.0 | tail -n 1)
mkdir -p engine/Sandbox.Compiling/EmbeddedRefs
cp "$ref"/*.dll engine/Sandbox.Compiling/EmbeddedRefs/

dotnet build engine/Launcher/SboxServer/Sbox-Server.csproj -c Release -v q -p:ApplicationIcon=

# SkiaSharp's and HarfBuzz's Linux natives (MIT, NuGet)
mkdir -p game/bin/linuxsteamrt64 game/bin/managed
for pkg in skiasharp.nativeassets.linux harfbuzzsharp.nativeassets.linux; do
	v=$SKIA; [ "$pkg" = harfbuzzsharp.nativeassets.linux ] && v=8.3.0
	t=$(mktemp -d)
	curl -sSL "https://api.nuget.org/v3-flatcontainer/$pkg/$v/$pkg.$v.nupkg" -o "$t/p.zip"
	(cd "$t" && unzip -q p.zip 'runtimes/linux-x64/native/*')
	cp "$t"/runtimes/linux-x64/native/*.so game/bin/linuxsteamrt64/
	cp "$t"/runtimes/linux-x64/native/*.so game/bin/managed/
	rm -rf "$t"
done
# Material Icons, the icon font s&box's UI names (font-family: Material Icons;
# each icon a ligature of its name): Google's, Apache 2.0, not in sbox-public
# (it comes with Facepunch's download). The game loads every font under a
# mounted /fonts/.
icons=game/core/fonts/MaterialIcons-Regular.ttf
if [ ! -s "$icons" ]; then
	mkdir -p game/core/fonts
	curl -sSfL https://raw.githubusercontent.com/google/material-design-icons/master/font/MaterialIcons-Regular.ttf -o "$icons" || {
		rm -f "$icons"; echo "setup: Material Icons not fetched; icons will show as their names"; }
fi
echo "ready: SBOX=$SBOX sh spike.sh 90 +shimgame $SBOX/game/samples/sweeper/.sbproj"
