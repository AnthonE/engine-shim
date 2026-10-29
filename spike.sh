#!/bin/sh
# One turn of the loop: regenerate the slots (picking up impls.rs), build,
# put libengine2.so where s&box's managed dedicated server looks, boot it,
# and show where it stopped and which slots it called first.
#   SBOX=<a built sbox-public checkout> sh spike.sh [seconds] [server args...]
set -e
ROOT=$(pwd)
: "${SBOX:?SBOX=<sbox-public checkout, managed server built: see README>}"
OUT=${OUT:-target/spike}
secs=${1:-60}; [ $# -gt 0 ] && shift
mkdir -p "$OUT"
dotnet run --project gen -- "$SBOX" engine engine2/src/slots.rs
cargo build --release -q
mkdir -p "$SBOX/game/bin/linuxsteamrt64"
cp target/release/libengine2.so "$SBOX/game/bin/linuxsteamrt64/"
# the engine modules the managed side loads by name, all this one library
for m in vfx_vulkan filesystem_stdio; do ln -sf libengine2.so "$SBOX/game/bin/linuxsteamrt64/lib$m.so"; done
(cd "$SBOX/game" && ENGINE2_TRACE="$ROOT/$OUT/trace.txt" timeout "$secs" ./sbox-server "$@" > "$ROOT/$OUT/server.log" 2>&1) && code=0 || code=$?
echo "exit $code"
grep -v '^\s*at ' "$OUT/server.log" | tail -12
echo "-- $(grep -c '^first' "$OUT/trace.txt" || true) slots called; the last:"
grep '^first' "$OUT/trace.txt" | tail -8
