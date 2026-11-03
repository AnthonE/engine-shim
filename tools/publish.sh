#!/bin/sh
# Builds from tools/build.sh as the Elo Pros origin serves them (scry-forge
# meter/launcher.py): a stage to copy onto the origin's depot root, the
# pointer that makes them live, and the title's manifest.
#   sh tools/publish.sh <slug> <name> <stage dir> <build dir>... [--origin https://elopros.com] [--repo URL] [--blurb TEXT]
#   sh tools/publish.sh sandbox Sandbox ../stage ../sandbox-build ../sandbox-win
# Writes, into <stage>:
#   depots/<slug>/<build>/<platform>/   depot.json and the bytes beside it, for $SCRY_DEPOTS_DIR
#   depots/<slug>/published.json         {"<platform>": "<build>"}: copied LAST, it is what goes live
#   <slug>.manifest.json                 for scry-forge's data/launcher/ (a native slot per platform)
#   PUBLISH.txt                          the copy, in order
# Each depot's root is on the origin (the route that serves it), so the
# notary's digest and the download agree; `elo digest` computes it.
set -e
ROOT=$(pwd)
slug=${1:?a slug (lowercase letters, digits, - and _)}; name=${2:?a name for the title}; stage=${3:?a stage directory}; shift 3
origin=https://elopros.com; repo=; blurb=; builds=
while [ $# -gt 0 ]; do
	case $1 in
		--origin) origin=${2%/}; shift 2 ;;
		--repo) repo=$2; shift 2 ;;
		--blurb) blurb=$2; shift 2 ;;
		-*) echo "publish: unknown argument $1" >&2; exit 2 ;;
		*) builds="$builds $(cd "$1" && pwd)"; shift ;;
	esac
done
[ -n "$builds" ] || { echo "publish: name at least one build from tools/build.sh" >&2; exit 2; }
rm -rf "$stage"; mkdir -p "$stage/depots/$slug"; stage=$(cd "$stage" && pwd)

pointer=; slots=; sep=
for b in $builds; do
	platform=$(sed -n 's/.*, for \([a-z0-9_-]*\)$/\1/p' "$b/BUILD.txt")
	[ -n "$platform" ] || platform=linux-x86_64   # a build from before BUILD.txt named it
	rev=$(sed -n 's/^engine-shim \([0-9a-f]*\).*/\1/p' "$b/BUILD.txt")
	version=0.1.0-g$rev
	grep -q '^engine-shim .*uncommitted' "$b/BUILD.txt" && version=$version-dirty
	base=$origin/api/launcher/depot/$slug/$version/$platform
	t=$stage/.tmp-$platform
	python3 "$ROOT/tools/depot.py" "$b" "$t" --slug "$slug" --name "$name" --platform "$platform" \
		--version "$version" --base "$base" --servers "$origin/api/launcher/servers/$slug" \
		${repo:+--repo "$repo"} ${blurb:+--blurb "$blurb"}
	# the origin's layout: the bytes beside depot.json, at their depot paths
	d=$stage/depots/$slug/$version/$platform
	mkdir -p "$(dirname "$d")"
	mv "$t/files" "$d"
	mv "$t/depot.json" "$d/depot.json"
	rm -rf "$t"
	pointer="$pointer$sep\"$platform\": \"$version\""
	slots="$slots$sep{\"kind\": \"native\", \"platform\": \"$platform\", \"depot\": null, \"version\": null, \"bytes\": null, \"notes\": \"an s&box game on engine-shim (github.com/AnthonE/engine-shim): the .NET runtime, s&box's MIT engine and this library, no closed binary\"}"
	sep=", "
done
echo "{$pointer}" > "$stage/depots/$slug/published.json"

python3 - "$stage/$slug.manifest.json" "$slug" "$name" "$origin" "$repo" "$blurb" "[$slots]" <<'PY'
import json, sys
out, slug, name, origin, repo, blurb, slots = sys.argv[1:]
json.dump({
    "manifest_version": 1, "slug": slug, "name": name, "publisher": "Elo Pros",
    "state": "in-build",
    "_state": "a floor: the origin serves the listing's state when the shelf names this title",
    "kind": "an s&box game on engine-shim", "blurb": blurb, "repo": repo, "source_required": bool(repo),
    "_note": "native rows are slots the origin fills from $SCRY_DEPOTS_DIR/<slug>/published.json",
    "builds": json.loads(slots),
    "servers": {"url": f"{origin}/api/launcher/servers/{slug}", "kind": "elo-shardlist-v1"},
}, open(out, "w"), indent=2)
PY

cat > "$stage/PUBLISH.txt" <<EOF
$name ($slug), staged $(date -u +%Y-%m-%dT%H:%M:%SZ) for $origin

On the origin box, in this order (a build is live when published.json names
it, never because its directory is there, so the pointer goes last):

1. the bytes:    rsync -a $stage/depots/$slug/ <box>:\$SCRY_DEPOTS_DIR/$slug/ --exclude published.json
2. the manifest: $slug.manifest.json into scry-forge's data/launcher/, committed, pulled on the box
3. live:         rsync -a $stage/depots/$slug/published.json <box>:\$SCRY_DEPOTS_DIR/$slug/published.json
                 (if the title already has one, merge: its other platforms and branches stay)
4. look:         curl -s $origin/api/launcher/manifest/$slug   (each native row: depot_state "published")
                 elo install $slug && elo verify $slug
5. the seal:     elo digest <depot.json> per platform, then the notary's notarize(digest, label, memo)

The digests:
EOF
for d in "$stage"/depots/"$slug"/*/*/depot.json; do
	echo "  $(echo "$d" | sed "s#$stage/depots/##")  $(python3 -c "import json,sys;d=json.load(open(sys.argv[1]));print(len(d['files']),'files',round(sum(f['bytes'] for f in d['files'])/1e6),'MB')" "$d")" >> "$stage/PUBLISH.txt"
done
echo "staged $slug for $origin: $stage (see PUBLISH.txt)"
