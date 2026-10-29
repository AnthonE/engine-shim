#!/bin/sh
# A map package's paks from sbox.game, into a project: the newest revision
# whose files the CDN holds (a map's newest may be on Steam's workshop
# only, which needs Steam). Only the paks, not the textures beside them.
# Facepunch's content (or its creator's), under their terms: fetched for
# your own server, never committed here.
#   sh tools/map.sh facepunch.flatgrass ../sandbox/Assets
set -e
ident=${1:?a map package, org.ident}; out=${2:?the Assets folder of a project}
mkdir -p "$out/maps"
python3 - "$ident" "$out" <<'PY'
import json, os, sys, urllib.request
ident, out = sys.argv[1], sys.argv[2]
api = "https://public.facepunch.com/sbox/package"
get = lambda url: json.load(urllib.request.urlopen(url, timeout=60))
for rev in get(f"{api}/versions/2/{ident}"):
    files = get(rev["ManifestUrl"]).get("Files", [])
    paks = [f for f in files if f["path"].endswith(".vpk")]
    if not paks: continue
    for f in paks:
        dest = os.path.join(out, f["path"])
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        if not (os.path.exists(dest) and os.path.getsize(dest) == f["size"]):
            urllib.request.urlretrieve(f["url"], dest)
        print(f"{ident} revision {rev['Id']} ({rev['Created'][:10]}): {f['path']}, {f['size']} bytes")
    break
else:
    sys.exit(f"{ident}: no revision with its paks on the CDN")
PY
