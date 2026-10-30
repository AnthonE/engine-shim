#!/bin/sh
# A fresh machine made ready for spike.sh: sbox-public at our commit, patched,
# its interop generated, the managed dedicated server built, and the MIT
# natives and reference assemblies it needs beside it. Run from the
# repository's root; the checkout goes beside it (../sbox-public).
#   sh tools/setup.sh [sbox-public dir]
set -e
ROOT=$(pwd)
SBOX=${1:-$ROOT/../sbox-public}
export DOTNET_ROOT=${DOTNET_ROOT:-$HOME/.dotnet} PATH=$HOME/.dotnet:$PATH
SKIA=3.116.0

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
echo "ready: SBOX=$SBOX sh spike.sh 90 +shimgame $SBOX/game/samples/sweeper/.sbproj"
