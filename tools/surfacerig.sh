#!/bin/sh
# Sweeper's scene as a project of its own, with tools/surfacerig/SurfaceRig.cs
# in it: balls, boxes and a floor made of core's surfaces (compile those
# first: sh tools/compile.sh <sbox-public>/game/core surface), and once a
# second how they bounce, slide, weigh and trace.
#   sh tools/surfacerig.sh <sbox-public checkout> <out dir>
#   SBOX=../sbox-public sh spike.sh 20 +shimgame <out dir>/.sbproj
#   grep surfacerig target/spike/server.log
set -e
sbox=${1:?an sbox-public checkout}; out=${2:?where to make the project}
here=$(cd "$(dirname "$0")" && pwd)
rm -rf "$out" && cp -r "$sbox/game/samples/sweeper" "$out"
rm -rf "$out/obj" && find "$out" -name "*_c" -delete
sed -i 's/"Ident": "sweeper"/"Ident": "surfacerig"/; s/"Title": "Sweeper"/"Title": "Surfacerig"/' "$out/.sbproj"
cp "$here/surfacerig/SurfaceRig.cs" "$out/code/"
python3 - "$out/Assets/scenes/minimal.scene" <<'PY'
import json, sys, uuid
p = sys.argv[1]
d = json.load(open(p))
d["GameObjects"].append({"__guid": str(uuid.uuid4()), "Flags": 0, "Name": "SurfaceRig", "Enabled": True,
                         "Components": [{"__type": "SurfaceRig", "__guid": str(uuid.uuid4())}]})
json.dump(d, open(p, "w"), indent=2)
PY
sh "$here/compile.sh" "$out"
