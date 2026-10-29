#!/bin/sh
# Sweeper's scene (72 physics cubes on a plane) as a project of its own,
# with tools/physdrop/Watch.cs in it: once a second it logs what physics is
# doing (bodies awake, their heights, a trace down, collisions heard), and
# kicks a cube up at six seconds.
#   sh tools/physdrop.sh <sbox-public checkout> <out dir>
#   SBOX=../sbox-public sh spike.sh 25 +shimgame <out dir>/.sbproj
#   grep watch target/spike/server.log
set -e
sbox=${1:?an sbox-public checkout}; out=${2:?where to make the project}
here=$(cd "$(dirname "$0")" && pwd)
rm -rf "$out" && cp -r "$sbox/game/samples/sweeper" "$out"
rm -rf "$out/obj" && find "$out" -name "*_c" -delete
sed -i 's/"Ident": "sweeper"/"Ident": "physdrop"/; s/"Title": "Sweeper"/"Title": "Physdrop"/' "$out/.sbproj"
cp "$here/physdrop/Watch.cs" "$out/code/"
python3 - "$out/Assets/scenes/minimal.scene" <<'PY'
import json, sys, uuid
p = sys.argv[1]
d = json.load(open(p))
d["GameObjects"].append({"__guid": str(uuid.uuid4()), "Flags": 0, "Name": "Watch", "Enabled": True,
                         "Components": [{"__type": "Watch", "__guid": str(uuid.uuid4())}]})
json.dump(d, open(p, "w"), indent=2)
PY
sh "$here/compile.sh" "$out"
