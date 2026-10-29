# engine-shim

An open native engine under s&box's open C# engine. s&box's engine layer is
MIT (github.com/Facepunch/sbox-public): the scene system, networking,
UI, hotloading, the compiler, the dedicated server. It runs on a closed
core, `engine2`, under the s&box EULA. A game only talks to the C#. This
repository reimplements the interface between them, in Rust, so the same
C# and the games built on it can run with no closed binary under them: on
our own servers, on Linux, and later in a browser.

This is research. Nothing here is a product yet.

## How the two halves meet

Measured on sbox-public at the commit in `SBOX_PUBLIC_COMMIT`:

- The launcher is plain .NET. It loads `bin/<platform>/libengine2.so` with
  `NativeLibrary` and calls one export, `igen_engine(hash, managed, native,
  structSizes)`. Native fills `native` with a function pointer per slot and
  keeps `managed`, the callbacks it may call back into.
- The table is written by Facepunch's own generator (`engine/Tools/InteropGen`,
  MIT) from the `.def` files in `engine/Definitions` (MIT). The same
  generator writes the C++ side, so the slot order is exact.
- The runtime table, `igen_engine`, has 3,578 slots: about 3,000 functions,
  548 field accessors and the casts between classes, plus 71 managed
  callbacks the other way. The editor is a separate table, `igen_tools`,
  with 4,773 slots (the Qt layer the editor's C# draws with). Hammer,
  ModelDoc and AnimGraph's editors add 642, 31 and 3. A game or a server
  needs only `igen_engine`.
- Besides `engine2`, the managed side calls Valve's `steam_api` directly
  (Steam networking, lobbies, identity). Its shader compiler loads
  `vfx_vulkan`. Those need answers too.

## What runs now

Facepunch's managed dedicated server (`engine/Launcher/SboxServer`) builds
on Linux from the public repo and NuGet alone. On `libengine2.so` from this
repository it boots as a dedicated server, compiles and loads a game, and
runs it:

```
engine-shim: dedicated=True editor=False headless=True
Loading game 'local.sweeper#local'
game.load success local.sweeper#local 2581ms [... assemblies=60, resources=154, fonts=12, scene=454]
```

That is Facepunch's Sweeper sample: its C# compiled in the engine, its
scene loaded, its components started. 131 of the 3,578 slots are called on
the way; 33 are written by hand (`engine2/src/impls.rs`), among them a null
audio device, the empty render device Facepunch's engine already runs a
server on (`RENDER_DEVICE_API_EMPTY`), and compiled resources read as
plain JSON. Every other slot is a generated stub that records its first
call and returns zeroes, or a fresh zeroed block where it returns a pointer.

Native objects of the handle classes (scene objects, worlds, lights,
physics bodies) are made and registered: every `Create…` slot that returns
a handle calls managed's `HandleIndex.RegisterHandle(pointer, type)` with
StringToken's type token (`engine2/src/handles.rs`), so the physics world
and the colliders now exist. Casts between classes return the same object.

The way in for players is written. A dedicated server logs on to Steam
before it opens its sockets; here logging on succeeds without Steam
(`ISteamGameServer_LogOn…`, answered with the `SteamServersConnected`
callback). The server then listens with Facepunch's own networking code
over our transport (`engine2/src/net.rs`): the 29 `Glue.Networking` slots,
which hold the listen sockets, connections, poll groups and messages in the
layout managed reads, carried over QUIC through WebTransport on the
server's port (27015/udp), as Gates' shards speak it. Reliable messages
ride one stream in order; the rest go as datagrams, dropped rather than
waited for. A browser can reach the same port one day.

Players are who they prove they are (`engine2/src/steam.rs`). The server
welcomes each connection with a fresh nonce; the player's Elo Pros
launcher signs it in with Ethereum (`prove`, EIP-4361, over the address
they dialled), and that proof is the ticket managed hands
`BeginAuthSession`. The server recovers the signer (`engine2/src/siwe.rs`:
keccak and secp256k1, no network call), checks the domain, the nonce (once)
and its age, and answers `ValidateAuthTicketResponse` as Steam would. A
player's id is their address's first 8 bytes, where Steam has a SteamID.
With no launcher a player has a guest ticket.

| | |
| --- | --- |
| `ENGINE2_SIWE_DOMAINS` | the names a server answers to (commas). Set: only a sign-in for one of them gets in. Unset: a development server, which takes guests at their word |
| `ENGINE2_TLS_CERT`, `ENGINE2_TLS_KEY` | the server's certificate (PEM). Unset: a self-signed one, its hash printed at start (valid 14 days, WebTransport's rule) |
| `ENGINE2_CERT_HASH` | a player pins a self-signed server by that hash. Without it a player checks the system's roots, and takes any certificate on loopback only: the sign-in has no channel binding, so an unchecked relay could sign in as the player |
| `ENGINE2_ELO_GAME` | the title's slug in the Elo Pros catalog: what the launcher signs for, and what copies are checked against (default `engine-shim`) |
| `ENGINE2_IDENTITY` | an id to be, over the launcher's |

A server is on the store the way Gates' shards are (`engine2/src/elo.rs`,
after elo-pros-launcher's `ROOMS-SERVER.md` §4). The name, map and cap
managed hands Steam's game server become a shard-list row
(`elo-shardlist-v1`), a status endpoint answers the launcher's count, and
on a title that sells copies a signed-in player's wallet is asked about at
join and the roster every two minutes: a definite "no copy" refuses or
kicks (the player hears why), and "could not look" never does.

| | |
| --- | --- |
| `ENGINE2_PUBLIC_ADDR` | `host:port` players reach this server at: the row's `addr`, and the `elo://join/<slug>/<addr>` link printed at start |
| `ENGINE2_SHARDLIST` | where to write the row, kept current; the origin serves `$SCRY_DEPOTS_DIR/<slug>/servers.json` as written |
| `ENGINE2_SHARD_ID` | the row's `id` (default: the address) |
| `ENGINE2_STATUS_ADDR` | where `GET /status.json` answers, plain HTTP (`{"players":…,"max_players":…,"tick":…}`), and beside it `GET /online.json`, who is signed in and since when: the store's playtime and friends' "playing now" read it |
| `ENGINE2_STATUS_URL` | where the world reaches it, for the row's `status_url` (https, behind your proxy) |
| `ENGINE2_ELO_ORIGIN` | `https://elopros.com` arms the copy check (`/api/ticket/<slug>/…`). Unset: nothing is checked |

Measured under the real server (Sweeper, hosted with `+shimhost`): it logs
on, prints `Server started!`, listens on QUIC, and writes its shard row
from what managed set (`"Sweeper Dedicated Server"`, map `Sweeper`, 64
players). A player dialling in (`net.rs`'s `probe`, below) is accepted by
managed's own `ShouldAcceptConnection`, welcomed with a nonce, counted in
`status.json`, and sent s&box's first handshake message, the server's info,
compressed with LZ4 (the eight `LZ4Glue` slots, `lz4_flex` behind them).

**And a player joins.** The same launcher runs as a headless s&box player
(`+shimclient <address>`), on this library too, and joins with s&box's own
`connect`: the handshake, the game's code sent over the network (Sweeper
is a local project), the scene's snapshot. With a launcher signing in
(here the stand-in below) and a server that takes only a sign-in for its
address, the player is their wallet's id, the server checks the proof,
says `[17865156273432703638] is connected`, and `online.json` names the
wallet. The scene comes up on both sides with no error: every component
enables (the object state below).

```sh
# a stand-in launcher (test key 9), a server that wants sign-ins, a player
ELO_LAUNCHER_SOCKET=/tmp/l.sock cargo test --release -- --ignored stand_in_launcher &
ENGINE2_SIWE_DOMAINS=127.0.0.1:27015 ENGINE2_STATUS_ADDR=127.0.0.1:27016 \
  SBOX=../sbox-public sh spike.sh 140 +shimgame $PWD/../sbox-public/game/samples/sweeper/.sbproj +shimhost &
# once "Server started!":
(cd ../sbox-public/game && ELO_LAUNCHER_SOCKET=/tmp/l.sock ./sbox-server +shimclient 127.0.0.1:27015 +instanceid 2)
curl -s 127.0.0.1:27016/online.json
```

```sh
ENGINE2_PUBLIC_ADDR=127.0.0.1:27015 ENGINE2_STATUS_ADDR=127.0.0.1:27016 \
  SBOX=../sbox-public sh spike.sh 90 +shimgame $PWD/../sbox-public/game/samples/sweeper/.sbproj +shimhost
# once it says "Server started!":
curl -s 127.0.0.1:27016/status.json
(cd engine2 && ENGINE2_PROBE=127.0.0.1:27015 cargo test --release -- --ignored probe --nocapture)
```

Players can also reach a server by its identity rather than its address,
through the relay (`engine2/src/relay.rs`, the `relay` binary): a server
behind a NAT, or one hiding its address, registers there, and s&box's own
`ConnectToSteamId` finds it. A server proves its identity with its host
key, so nobody can register as someone else's server; the relay pipes each
player's stream to the server untouched, so the handshake, the nonce and
the sign-in run end to end, and it reads none of it. Measured: the real
server, registered through the relay with a host key it made, takes the
probe dialling it by identity, and sends the same first message.

| | |
| --- | --- |
| `ENGINE2_RELAY` | `host:port` of a relay. A server registers its identity socket there; a player's `ConnectToSteamId` goes through it |
| `ENGINE2_HOST_KEY` | a server's key file (64 hex digits, made on first start, 0600): its identity across restarts, proven to the relay |
| `ENGINE2_RELAY_PORT` | the `relay` binary's port (default 27080, udp); its certificate is a server's (`ENGINE2_TLS_CERT`/`KEY`, or self-signed and pinned) |
| `ENGINE2_RELAY_METER`, `ENGINE2_RELAY_SECRET` | the relay asks the Elo Pros meter (`https://elopros.com/api`, scry-forge's `meter/gamerelay.py`) whether a server may register, and beats each minute with its players: billed by the player-hour from its developer's credits, and dropped when they run out. Unset: every server carried free, a relay for your own |

A developer puts a server on their bill with `relay claim <host key file>
<account>`: its address and its host key's signature over the claim, for
the dev console. Measured against scry-forge's own meter, served alone:
the claim lands, the server registers through the relay, a player joins it
by identity with s&box's `connect <identity>`, and the first beat lands one
ledger line (`gamerelay:<session>:0`, 1¢ for ten minutes).

```sh
cargo build --release && ENGINE2_RELAY_PORT=27080 ./target/release/relay &
ENGINE2_RELAY=127.0.0.1:27080 ENGINE2_HOST_KEY=$PWD/host.key \
  SBOX=../sbox-public sh spike.sh 90 +shimgame $PWD/../sbox-public/game/samples/sweeper/.sbproj +shimhost
# its identity is the SteamID "Server started!" prints:
(cd engine2 && ENGINE2_PROBE=127.0.0.1:27080 ENGINE2_PROBE_ID=<identity> cargo test --release -- --ignored probe --nocapture)
```

A sign-in through a relay names the relay's address, so a server behind
one lists the relay's name in `ENGINE2_SIWE_DOMAINS`.

The rest is tested the way managed calls it (`cargo test`): a proof a
stand-in launcher signed, a player without a copy refused at the door by a
stand-in origin, a kicked one hearing why.

Native objects remember (`engine2/src/state.rs`, `handles.rs`), and gen/
writes all of it from the managed interop:

- **What they were made with.** Every create that hands managed a handle
  registers the object as its class (a light, a body, a shape, a joint;
  the `Add…` slots of a physics world and body too), and keeps its pointer
  arguments. A getter that answers with another object's handle (`GetWorld`,
  `GetBody`, `GetParent`) gives the one of that type it was made with.
- **What each `Set…` stored, for its `Get…`.** A setter and a getter of
  the same name and value type (passed by value or by pointer to a struct,
  returned by value or through an out pointer), and every field accessor
  pair: 798 slots, 339 properties. A raw pointer is never kept (a string,
  a buffer: managed's, gone after the call).

Measured: Sweeper's scene loads on the server, and on a player who joins
it, with no error at all, and the physics world steps on both.

**Things move** (`engine2/src/physics.rs`). Where s&box's native engine
runs Box3D, worlds, bodies and shapes here are Rapier's (MIT/Apache-2.0,
pure Rust, so the browser too), in s&box's units (inches, kilograms;
Rapier's tolerances told so). 157 slots are written, the physics ones and
the result vectors they fill:

- **Bodies**: static, keyframed (sent to a target each step) and dynamic;
  position, rotation, velocities, impulses and forces (spent each step, as
  Box3D's are), damping, gravity and its scale, motion locks, sleep, CCD.
  Mass comes from the shapes at water's density, unless managed sets a
  mass, a centre or an inertia over it; mass updates defer while a
  collider builds its shapes.
- **Shapes**: spheres, capsules, boxes, hulls, triangle meshes and height
  fields, each with its tags, its surface, friction, elasticity, and
  whether it is a trigger, ignores traces or weighs nothing.
- **The step**: `StepSimulation` steps, then tells managed each body that
  moved (`PhysicsEngine.OnActive`: where, how fast, whether it fell
  asleep), which is how a `Rigidbody` follows its body.
  `ProcessIntersections` hands over touches begun, kept and ended and
  triggers entered and left, in managed's notification layout, each
  shape's collision mask deciding what it hears.
- **Collision rules**: the project's `CollisionRules` (the JSON managed
  sends) decide per pair of tag sets whether two shapes collide, trigger
  or ignore each other, by managed's own rule (the least colliding pair,
  else the tags' defaults, else collide).
- **Traces**: rays, and spheres, boxes, capsules, cylinders and cones
  swept, or a body's own shapes; the nearest hit or all of them, through
  managed's tag filters (require, any, exclude), its trigger and object-set
  masks and its filter delegate, with the hit's body, shape, surface, tags
  and triangle. Also traces against one given shape, and `FindInPhysics`'s
  sphere, box and frustum queries.
- **Handles**: bodies and shapes removed free their managed handles, so
  managed's objects stop being valid with them.

Measured under the real server: Sweeper's scene has 72 physics cubes
stacked on a plane. With `tools/physdrop.sh`'s watcher in it, all 72 start
awake, settle (the lowest at z 14, half a cube) and are asleep within two
seconds; managed hears 150 collisions begin; a trace down from above hits
the top cube, its normal up; a cube kicked up at six seconds (managed's
`ApplyImpulse`) wakes 30 of them and they settle and sleep again. No error
on the server, nor on a player who joins it (whose own world steps,
traces and queries too).

```sh
sh tools/physdrop.sh ../sbox-public /tmp/physdrop
SBOX=../sbox-public sh spike.sh 25 +shimgame /tmp/physdrop/.sbproj
grep watch target/spike/server.log
```

Also tested the way managed calls it (`cargo test`): a ball dropped
through the slots alone falls, lands, is heard moving and falling asleep,
touches the floor, is traced, queried, filtered past, knocked back up and
removed.

What the game's components still can't do:

- **Joints.** Made and remembered (102 slots), not simulated yet:
  Rapier's fixed, spherical, revolute, prismatic and spring joints map
  onto most of them.
- **Model physics.** A model's collision (`CreateAggregateInstance`, the
  hulls and meshes a model carries) waits for the assets step.

## Reproduce

```sh
# .NET 10 SDK and Rust
git clone https://github.com/Facepunch/sbox-public && cd sbox-public
git checkout $(cat ../engine-shim/SBOX_PUBLIC_COMMIT)
git apply ../engine-shim/sbox-public.patch     # below
# the interop files: Facepunch's generator, run once (gen/ needs them)
#   see "Generating the interop" below
DOTNET_ROOT=~/.dotnet dotnet build engine/Launcher/SboxServer/Sbox-Server.csproj -c Release -p:ApplicationIcon=
# SkiaSharp and HarfBuzz's Linux natives (MIT, NuGet's *.NativeAssets.Linux)
#   into game/bin/linuxsteamrt64/ and game/bin/managed/
# .NET's reference assemblies (MIT, the SDK's packs/Microsoft.NETCore.App.Ref/
#   <version>/ref/net10.0/*.dll) into engine/Sandbox.Compiling/EmbeddedRefs/
#   before the build: the engine's C# compiler embeds them
cd ../engine-shim
sh tools/compile.sh ../sbox-public/game/samples/sweeper      # its scene as engine2 reads it
SBOX=../sbox-public sh spike.sh 90 +shimgame $PWD/../sbox-public/game/samples/sweeper/.sbproj
```

`spike.sh` regenerates `engine2/src/slots.rs` (`gen/`), builds, installs
`libengine2.so`, boots the server and prints where it stopped and the
slots it called first (`target/spike/trace.txt`, `server.log`).

**Generating the interop.** InteropGen is a library. A five-line console
project that references `engine/Tools/InteropGen/InteropGen.csproj` and
calls `Facepunch.InteropGen.Program.ProcessManifest("engine", false)` from
the checkout's root writes `engine/Sandbox.Engine/Interop.Engine.cs` and
the C++ under `src/`. The folders the `.def` files name for C++ must exist
first.

**sbox-public.patch** changes two things locally, neither shipped:

- `System.Speech` comes from NuGet, not the Windows DLL their setup downloads.
- The server's launcher prints every managed log line to stdout (and says
  whether it is a dedicated server), and runs `+shimgame <project.sbproj>`
  through the `game` command 30 frames in. The engine runs `+game` itself,
  but only as a package to download from sbox.game; a local project goes
  through the command. A path there is the server's: give it absolute.
- `+shimhost` runs `host` 300 frames later: it opens the lobby, as a
  networked game's `NetworkHelper` does for itself.
- `+shimclient <address>` boots the same launcher as a headless player (a
  game app, not a dedicated server) and runs `connect <address>`.
- A dedicated server running a local project counts as a developer host
  (`NetworkSystem.IsDeveloperHost`). It already sends such a game's code
  to its players (`GameInstanceDll`'s code archives), but told them it was
  no developer host, so they looked for a published package and failed.
  This one is a fix upstream could take.

`tools/compile.sh` copies a project's scenes and prefabs to their `_c`
names, which is all a JSON resource needs here. Models, materials and
textures are Source 2's binary formats and wait for the assets step.

No EULA binary is used: the checkout's `game/bin` holds only text files,
and Facepunch's artifact download (`Setup.sh`) is never run.

## Layout

| | |
| --- | --- |
| `gen/` | C#: writes `engine2/src/slots.rs` (the stubs, the creates, the remembered properties) from the managed interop (the slot order) and the built `Sandbox.Engine.dll` by reflection (every argument and return type, every struct's layout, its size asserted against the managed `sizeof` that Facepunch's C++ checks) |
| `engine2/src/lib.rs` | `igen_engine`: fills the table |
| `engine2/src/slots.rs` | generated: 3,578 typed slots, 81 struct layouts, the names |
| `engine2/src/state.rs` | the properties native objects remember, set by one slot and read by another |
| `engine2/src/impls.rs` | the slots written by hand; a `pub unsafe fn` named as a slot replaces its stub |
| `engine2/src/handles.rs` | handle objects: made, registered with managed, found by pointer, freed |
| `engine2/src/physics.rs` | physics on Rapier: worlds, bodies, shapes, the step and what touched, collision rules, traces and queries |
| `engine2/src/net.rs` | connections: Glue.Networking's sockets, poll groups and messages over QUIC (WebTransport) |
| `engine2/src/steam.rs` | Steam's part without Steam: logon, identity, sign-in tickets, the callbacks managed waits on |
| `engine2/src/relay.rs`, `src/bin/relay.rs` | the relay: servers register by identity, players dial them through it |
| `engine2/src/elo.rs` | the store's side of a server: the shard-list row, `status.json`, copies checked at join and on a sweep |
| `engine2/src/siwe.rs` | Sign-In With Ethereum, checked: the signer recovered, the message's fields read |
| `engine2/src/elo_overlay.rs` | the Elo Pros launcher's client, vendored as its SDK asks (elo-pros-launcher `sdk/rust/elo_overlay.rs`) |
| `engine2/src/trace.rs` | which slots are called, in order, and how often |
| `spike.sh` | one turn of the loop |
| `tools/compile.sh` | a project's JSON resources (scenes, prefabs) as compiled resources |
| `tools/physdrop.sh`, `tools/physdrop/` | Sweeper's physics cubes with a watcher that logs what physics does |

## Next

1. **A server that runs a game.** The handle protocol and physics are
   here (above); next joints, then a real open-source s&box game: its
   trace is the measured size of "a server", the slots one game needs.
2. **Steam's part.** Connections, a server's logon, the Elo Pros sign-in,
   the shard list and the relay are ours now, and a player joins (above).
   Next: lobbies. A relay run for others wants a price, as the stream relay's
   credits do (scry-forge `meter/relay.py`).
3. **Assets.** Games ship compiled Source 2 resources. ValveResourceFormat
   (MIT) reads models, materials, textures and maps. A server needs maps
   and collision first; shaders only matter to a client.
4. **A client.** Rendering: about 670 scene and render slots onto a renderer
   of our own choosing (Bevy or Godot; see below), then sound and input.
5. **The browser.** .NET and Rust both build to WebAssembly. Expect a large
   download.

## Open questions

- **Licences beyond the code.** The C# and `.def` files are MIT. Facepunch's
  player models and clothing (Citizen) are under their EULA, as are assets
  a game pulls from sbox.game's library. A game runs here only with assets
  its creator owns, and only with that creator's opt-in.
  `Facepunch.ActionGraphs` (NuGet) declares no licence; ask before bundling it.
- **The creator's own terms** with Facepunch decide where their game may
  run; read them for each game.
- **The renderer.** Bevy (Rust, MIT/Apache, wasm) fits a Rust `engine2`.
  Godot (MIT) brings an editor, Jolt physics and C#; whether it embeds
  under a .NET host cleanly is to be measured. Decide once a server runs
  and the client slots are measured.
- **The name.** s&box is Facepunch's; nothing we ship uses it as ours.

The `.def` files, the generated interop and the slot names are Facepunch
Studios', under the MIT licence (NOTICE).
