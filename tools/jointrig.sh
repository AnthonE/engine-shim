#!/bin/sh
# Sweeper's scene as a project of its own, with tools/jointrig/JointRig.cs
# in it: five rigs made of s&box's joint components (a hinged pendulum, a
# ball-jointed chain, a spring, a slider, a weld too weak to hold), and
# once a second what each is doing.
#   sh tools/jointrig.sh <sbox-public checkout> <out dir>
#   SBOX=../sbox-public sh spike.sh 20 +shimgame <out dir>/.sbproj
#   grep jointrig target/spike/server.log
set -e
sbox=${1:?an sbox-public checkout}; out=${2:?where to make the project}
here=$(cd "$(dirname "$0")" && pwd)
rm -rf "$out" && cp -r "$sbox/game/samples/sweeper" "$out"
rm -rf "$out/obj" && find "$out" -name "*_c" -delete
sed -i 's/"Ident": "sweeper"/"Ident": "jointrig"/; s/"Title": "Sweeper"/"Title": "Jointrig"/' "$out/.sbproj"
cp "$here/jointrig/JointRig.cs" "$out/code/"
python3 - "$out/Assets/scenes/minimal.scene" <<'PY'
import json, sys, uuid
p = sys.argv[1]
d = json.load(open(p))
d["GameObjects"].append({"__guid": str(uuid.uuid4()), "Flags": 0, "Name": "JointRig", "Enabled": True,
                         "Components": [{"__type": "JointRig", "__guid": str(uuid.uuid4())}]})
json.dump(d, open(p, "w"), indent=2)
PY
sh "$here/compile.sh" "$out"
