#!/bin/sh
# Ape Tavern's Grubs (github.com/apetavern/grubs, MIT) made ready to host
# here: its two asset folders made one (Assets/ and assets/ are one folder
# on Windows, two on Linux), its compiled content fetched from its package
# on sbox.game (models, materials, textures: its creator's, under their
# terms, never committed here), its JSON resources compiled, and
# tools/grubs/ShimWatch.cs in its startup scene, which starts a match once
# a player has joined (a dedicated server is the host, so no player sees
# START) and says what the match does.
#   git clone https://github.com/apetavern/grubs ../grubs
#   sh tools/grubs.sh ../grubs ../content-grubs
#   ENGINE2_CONTENT=$PWD/../content-grubs SBOX=../sbox-public sh spike.sh 600 +shimgame $PWD/../grubs/.sbproj +shimhost
# SHIMWATCH=0 leaves the watcher out: the lobby waits for a host.
set -e
dir=${1:?a Grubs checkout}; content=${2:?a folder for its compiled content}
here=$(cd "$(dirname "$0")" && pwd)
watch=${SHIMWATCH:-1}
[ -d "$dir/assets" ] && cp -R -n "$dir/assets/." "$dir/Assets/" 2>/dev/null
[ -d "$content/models" ] || {
	sh "$here/package.sh" apetavern.grubs "$content" '(_c|\.vpk)$'
	sh "$here/package.sh" apetavern.gr_beach "$content" '(_c|\.vpk)$'
}
if [ "$watch" = 1 ]; then cp "$here/grubs/ShimWatch.cs" "$dir/Code/ShimWatch.cs"; else rm -f "$dir/Code/ShimWatch.cs"; fi
python3 - "$dir/Assets/scenes/beach.scene" "$watch" <<'PY'
import json, sys, uuid
path, watch = sys.argv[1], sys.argv[2] == "1"
d = json.load(open(path))
has = any(g.get("Name") == "ShimWatch" for g in d["GameObjects"])
if watch and not has:
    d["GameObjects"].append({"__guid": str(uuid.uuid4()), "Flags": 0, "Name": "ShimWatch", "Enabled": True,
                             "Components": [{"__type": "ShimWatch", "__guid": str(uuid.uuid4())}]})
elif not watch and has:
    d["GameObjects"] = [g for g in d["GameObjects"] if g.get("Name") != "ShimWatch"]
json.dump(d, open(path, "w"), indent=2)
PY
sh "$here/compile.sh" "$dir" scene prefab geq sound sdflayer vpcf
