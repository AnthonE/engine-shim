#!/bin/sh
# Facepunch's Sandbox (github.com/Facepunch/sandbox, MIT) made ready to host
# here: its dedicated server starts in one of its own scenes rather than the
# map it downloads (facepunch.flatgrass, from sbox.game), that scene gets
# tools/sandbox/ShimWatch.cs, and its JSON resources are compiled.
#   git clone https://github.com/Facepunch/sandbox ../sandbox
#   sh tools/sandbox.sh ../sandbox [scenes/zoo.scene]
#   SBOX=../sbox-public sh spike.sh 120 +shimgame $PWD/../sandbox/sandbox.sbproj +shimhost
#   grep shimwatch target/spike/server.log
set -e
dir=${1:?a Sandbox checkout}; scene=${2:-scenes/zoo.scene}
here=$(cd "$(dirname "$0")" && pwd)
cp "$here/sandbox/ShimWatch.cs" "$dir/Code/ShimWatch.cs"
python3 - "$dir" "$scene" <<'PY'
import json, sys, uuid
dir, scene = sys.argv[1], sys.argv[2]
p = f"{dir}/sandbox.sbproj"
proj = json.load(open(p))
proj["Metadata"]["DedicatedServerStartupScene"] = scene
json.dump(proj, open(p, "w"), indent=2)
s = f"{dir}/Assets/{scene}"
d = json.load(open(s))
if not any(g.get("Name") == "ShimWatch" for g in d["GameObjects"]):
    d["GameObjects"].append({"__guid": str(uuid.uuid4()), "Flags": 0, "Name": "ShimWatch", "Enabled": True,
                             "Components": [{"__type": "ShimWatch", "__guid": str(uuid.uuid4())}]})
    json.dump(d, open(s, "w"), indent=2)
PY
sh "$here/compile.sh" "$dir" scene prefab sent spawn spp tdef wdef bdef hdef btndef ldef smemit semit sndef surface sound
