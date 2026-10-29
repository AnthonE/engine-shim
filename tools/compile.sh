#!/bin/sh
# A game project's JSON resources as the engine reads them compiled: each
# scene, prefab and GameResource file under Assets/ copied beside itself as
# <name>_c (engine2 reads plain JSON as a compiled resource). Models,
# materials and textures are Source 2's binary formats and are not handled.
#   sh tools/compile.sh <project dir> [extension...]
set -e
dir=${1:?a project directory (with its .sbproj)}; shift
exts=${*:-scene prefab}
n=0
for e in $exts; do
  for f in $(find "$dir/Assets" -type f -name "*.$e"); do cp "$f" "${f}_c"; n=$((n + 1)); done
done
echo "$n resources compiled in $dir/Assets ($exts)"
