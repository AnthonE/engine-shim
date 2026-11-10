#!/bin/sh
# A package's files from sbox.game, as its newest revision on the CDN lists
# them, into a folder (or those whose path matches a pattern). Facepunch's
# content (or its creator's), under their terms: fetched for your own
# development, never committed here.
#   sh tools/package.sh facepunch.tv ../content
#   sh tools/package.sh facepunch.flatgrass ../sandbox/Assets '\.vpk$'
set -e
ident=${1:?a package, org.ident}; out=${2:?a folder}; pattern=${3:-.}
mkdir -p "$out"
python3 - "$ident" "$out" "$pattern" <<'PY'
import json, os, re, sys, time, urllib.request
ident, out, pattern = sys.argv[1], sys.argv[2], re.compile(sys.argv[3])
api = "https://public.facepunch.com/sbox/package"
def again(f, *a):
    # the CDN drops a connection now and then: each fetch a few times
    for attempt in range(6):
        try: return f(*a)
        except OSError:
            if attempt == 5: raise
            time.sleep(2 * (attempt + 1))
get = lambda url: again(lambda u: json.load(urllib.request.urlopen(u, timeout=60)), url)
for rev in get(f"{api}/versions/2/{ident}"):
    files = [f for f in get(rev["ManifestUrl"]).get("Files", []) if pattern.search(f["path"])]
    if not files: continue
    for f in files:
        dest = os.path.join(out, f["path"])
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        if not (os.path.exists(dest) and os.path.getsize(dest) == f["size"]):
            again(urllib.request.urlretrieve, f["url"], dest)
    print(f"{ident} revision {rev['Id']} ({rev['Created'][:10]}): {len(files)} files, {sum(f['size'] for f in files)} bytes")
    break
else:
    sys.exit(f"{ident}: no revision with matching files on the CDN")
PY
