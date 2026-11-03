#!/usr/bin/env python3
"""A build from tools/build.sh as the Elo Pros launcher installs it: a depot
(every file's path, size and sha256, and the one command that starts it) and
a manifest naming it (elo-pros-launcher docs/LAUNCHER.md §3). Writes, into
<out>:

  files/         the build, laid out to be served at the depot's root
  depot.json     the depot: served at <base>/depot.json
  manifest.json  the title, its native build row pointing at that depot

  python3 tools/depot.py <build dir> <out dir> --slug sandbox --name Sandbox \\
      --base https://example.com/sandbox/0.1.0 [--version 0.1.0] \\
      [--servers https://elopros.com/api/launcher/servers/sandbox] \\
      [--publisher NAME] [--repo URL] [--blurb TEXT]

Serve <out> at --base. Then `elo install <base>/manifest.json` (or the
manifest's path on disk), `elo verify`, `elo play`. The launcher starts the
build's `play` with the server the player chose ({server}, or `best` on the
title's shard list when none) and the title's list ({servers}).

A depot names files, not links: a symlink in the build (libengine2.so's
module names) becomes a copy. The digest a notary commits is the launcher's
to compute (`elo digest depot.json`), never a second implementation here.
"""
import argparse, hashlib, json, os, shutil, stat, sys

p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
p.add_argument("build"); p.add_argument("out")
p.add_argument("--slug", required=True); p.add_argument("--name", required=True)
p.add_argument("--base", required=True, help="the https URL <out> is served at")
p.add_argument("--version", help="the build's name (default: from BUILD.txt's engine-shim commit)")
p.add_argument("--platform", default="linux-x86_64")
p.add_argument("--servers", help="the title's shard list (elo-shardlist-v1)")
p.add_argument("--publisher", default=""); p.add_argument("--repo", default=""); p.add_argument("--blurb", default="")
a = p.parse_args()

build = os.path.abspath(a.build); out = os.path.abspath(a.out)
if not os.path.isfile(os.path.join(build, "play")):
    sys.exit(f"depot: {build} has no play: make it with tools/build.sh")
if not a.slug or not all(c.islower() or c.isdigit() or c in "-_" for c in a.slug):
    sys.exit(f"depot: bad slug {a.slug!r}: lowercase letters, digits, - and _")
base = a.base.rstrip("/")
version = a.version
if not version:
    with open(os.path.join(build, "BUILD.txt")) as f:
        rev = next((l.split()[1] for l in f if l.startswith("engine-shim ")), "0")
    version = f"0.1.0-g{rev}"

files_dir = os.path.join(out, "files")
if os.path.exists(files_dir): shutil.rmtree(files_dir)
os.makedirs(files_dir)

def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""): h.update(chunk)
    return h.hexdigest()

files = []
for dirpath, dirnames, filenames in os.walk(build):
    dirnames.sort()
    for name in sorted(filenames):
        src = os.path.join(dirpath, name)
        rel = os.path.relpath(src, build).replace(os.sep, "/")
        real = os.path.realpath(src)
        if not real.startswith(build + os.sep):
            sys.exit(f"depot: {rel} links outside the build ({real})")
        if not os.path.isfile(real): continue
        mode = os.stat(real).st_mode
        executable = bool(mode & stat.S_IXUSR)
        dest = os.path.join(files_dir, rel)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        try: os.link(real, dest)        # the same disk: no second copy
        except OSError: shutil.copy2(real, dest)
        files.append({"path": rel, "sha256": sha256(real), "bytes": os.path.getsize(real), "executable": executable})

depot = {
    "depot_version": 1,
    "slug": a.slug,
    "build": version,
    "platform": a.platform,
    "root": f"{base}/files",
    "files": files,
    "launch": {
        "exec": "play",
        # {server}: the one the player chose (empty: play joins the best on
        # the list); {servers}: the title's shard list, from the manifest
        "args": ["{server}", "--servers", "{servers}"],
        "cwd": ".",
        # what the launcher signs a sign-in for, and copies are checked against
        "env": {"ENGINE2_ELO_GAME": a.slug},
    },
}
total = sum(f["bytes"] for f in files)
manifest = {
    "manifest_version": 1,
    "slug": a.slug,
    "name": a.name,
    "publisher": a.publisher,
    "state": "in-build",
    "kind": "an s&box game on engine-shim",
    "blurb": a.blurb,
    "repo": a.repo,
    "source_required": bool(a.repo),
    "builds": [{
        "kind": "native", "platform": a.platform, "depot": f"{base}/depot.json",
        "version": version, "bytes": total,
        "notes": "an s&box game on engine-shim (github.com/AnthonE/engine-shim): the .NET runtime, s&box's MIT engine and this library, no closed binary",
    }],
}
if a.servers:
    manifest["servers"] = {"url": a.servers, "kind": "elo-shardlist-v1"}
for name, doc in (("depot.json", depot), ("manifest.json", manifest)):
    with open(os.path.join(out, name), "w") as f: json.dump(doc, f, indent=1)
print(f"depot {a.slug} {version}: {len(files)} files, {total / 1e6:.0f} MB -> {out} (serve it at {base})")
