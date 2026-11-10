#!/bin/sh
# Any s&box game's checkout made ready to host here: its dedicated server
# starts in the scene named (a game made for a listen server names none of
# its own), that scene gets tools/game/ShimWatch.cs (what the scene holds,
# and a player's view saved every ten seconds), and its JSON resources are
# compiled: scenes, prefabs, sounds, and every other kind named. The game's
# compiled content (models, materials, textures) comes from its package on
# sbox.game (tools/package.sh), its creator's, never committed here.
#   sh tools/game.sh <checkout> <scene, under Assets/> [resource extension...]
#   sh tools/package.sh <org.ident> ../content-x '(_c|\.vpk)$'
#   ENGINE2_CONTENT=$PWD/../content-x SBOX=../sbox-public sh spike.sh 600 +shimgame <checkout>/<name>.sbproj +shimhost
# SHIMWATCH=0 leaves the watcher out.
set -e
dir=${1:?a game checkout}; scene=${2:?its scene to start in, under Assets/}; shift 2
here=$(cd "$(dirname "$0")" && pwd)
watch=${SHIMWATCH:-1}
proj=$(ls "$dir"/*.sbproj "$dir"/.sbproj 2>/dev/null | head -n 1)
[ -n "$proj" ] || { echo "no .sbproj in $dir"; exit 1; }
python3 - "$dir" "$proj" "$scene" "$watch" "$here/game/ShimWatch.cs" <<'PY'
import json, os, shutil, sys, uuid
dir, proj, scene, watch, source = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4] == "1", sys.argv[5]
p = json.load(open(proj, encoding="utf-8-sig"))
p.setdefault("Metadata", {})["DedicatedServerStartupScene"] = scene
json.dump(p, open(proj, "w"), indent=2)
# its code folder, as the project names it (whatever its case on disk)
code = (p.get("CodePath") or "/code/").strip("/")
code = next((d for d in os.listdir(dir) if d.lower() == code.lower()), code)
target = os.path.join(dir, code, "ShimWatch.cs")
if watch: shutil.copy(source, target)
elif os.path.exists(target): os.remove(target)
s = os.path.join(dir, "Assets", scene)
d = json.load(open(s, encoding="utf-8-sig"))
has = any(g.get("Name") == "ShimWatch" for g in d["GameObjects"])
if watch and not has:
    d["GameObjects"].append({"__guid": str(uuid.uuid4()), "Flags": 0, "Name": "ShimWatch", "Enabled": True,
                             "Components": [{"__type": "ShimWatch", "__guid": str(uuid.uuid4())}]})
elif not watch and has:
    d["GameObjects"] = [g for g in d["GameObjects"] if g.get("Name") != "ShimWatch"]
json.dump(d, open(s, "w"), indent=2)
PY
sh "$here/compile.sh" "$dir" scene prefab sound "$@"
