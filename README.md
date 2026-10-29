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
Rapier's tolerances told so). 268 slots are written, the physics ones and
the result vectors they fill:

- **Bodies**: static, keyframed (sent to a target each step) and dynamic;
  position, rotation, velocities, impulses and forces (spent each step, as
  Box3D's are), damping, gravity and its scale, motion locks, sleep, CCD.
  Mass comes from the shapes at their surfaces' densities, unless managed
  sets a mass, a centre or an inertia over it; mass updates defer while a
  collider builds its shapes.
- **Shapes**: spheres, capsules, boxes, hulls, triangle meshes and height
  fields, each with its tags, its surface, friction, elasticity, and
  whether it is a trigger, ignores traces or weighs nothing.
- **Surfaces** (`engine2/src/surfaces.rs`): each `.surface` the game
  loads (`Sandbox.Surface`: core's wood, metal, rubber... and a project's
  own) is added by name and given its friction, elasticity and density;
  managed keeps the index it gets. A shape is made of one (by index or by
  name; none named is the default, `surfaces/default.surface`'s 0.8, 0.05
  and 2,000 kg/m³) and takes its friction and elasticity unless managed
  sets its own, and weighs its volume at its density. Two shapes touching
  mix as Box3D's do: friction the geometric mean, elasticity the greater.
  A mesh's or height field's triangles can each be of their own surface
  (a model's collision mesh, a terrain's painted materials): a contact
  takes the touching triangle's friction and elasticity, and a trace or a
  touch reports its surface. Nothing bounces off a contact slower than
  its surfaces' bounce threshold (Box3D's restitution threshold), so
  things come to rest. A surface reloaded reaches the shapes made of it,
  and their mass. A model file's shapes name their surfaces by hash,
  found by name when managed asks.
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
- **Joints** (`engine2/src/physics/joints.rs`): all nine of s&box's kinds,
  each one of Rapier's generic joints, on Box3D's axes (a hinge turns
  about its frame's z, a slider and a wheel's suspension run along its x):
  welds (rigid, or soft on their springs), springs and ropes (a range, a
  rest length, pulling or pushing only), hinges and sliders with limits,
  springs, motors and friction, ball joints with twist and swing limits,
  a target rotation or a motor, wheels (suspension, steering, a spinning
  motor), motor (control) joints, upright joints and filter joints. A
  joint pulled past its strength is told to managed once
  (`PhysicsEngine.OnPhysicsJointBreak`); a body removed takes its joints.
- **Handles**: bodies, shapes and joints removed free their managed
  handles, so managed's objects stop being valid with them.

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

And joints, by s&box's own joint components (`tools/jointrig.sh`): a
pendulum on a `HingeJoint` keeps to 100.0 from its pivot and to its plane
(y 0.00) as it swings from side to side; a chain of three `BallJoint`s
keeps its links 49.5 apart as it swings; a box on a `SpringJoint` (rest
100, 2 Hz) settles at 105.4, the 5.4 gravity's pull at that stiffness
predicts; a box on a vertical `SliderJoint` drops to its limit, -100,
and no way else; a `FixedJoint` with a break force of 1 breaks at once,
`OnBreak` fires, and its box falls to the floor.

```sh
sh tools/jointrig.sh ../sbox-public /tmp/jointrig
SBOX=../sbox-public sh spike.sh 12 +shimgame /tmp/jointrig/.sbproj
grep jointrig target/spike/server.log
```

Also tested the way managed calls it (`cargo test`): a ball dropped
through the slots alone falls, lands, is heard moving and falling asleep,
touches the floor, is traced, queried, filtered past, knocked back up and
removed. And surfaces: a ball of elasticity 0.9 dropped 182 comes back up
over 100 while the default's barely leaves the floor; a box pushed at 300
slides over 400 on ice (friction 0.01) and under 100 on the default; a
box of wood weighs 540 kg/m³ and twice that once the surface reloads; a
floor mesh half glue and half ice holds one box and lets the other slide,
traces and touches naming each half's surface; a height field's painted
cell traces as its own; a rubber ball set down gently doesn't bounce.

Measured under the real server with core's surfaces compiled (`sh
tools/compile.sh <sbox-public>/game/core surface`) and `tools/surfacerig.sh`'s
rig, made of s&box's own colliders on a floor of the default surface:

| | measured | predicted |
| --- | --- | --- |
| a ball dropped 180: default (0.05), rubber (0.4), metal sheet (0.25) | 0.2, 28.2, 10.8 | 0.45, 28.8, 11.25 (e²h) |
| a box pushed at 300: default, slippy_wheels, slippy_wheels with `Friction = 1` | 65.8, 591.5, 58.6 | 66.2, 592, 59.2 (v²/2μg, μ the geometric mean) |
| a 20 cube's `Rigidbody.Mass`: default, wood, metal, fabric | 262.2, 183.5, 354.0, 22.3 | the same (volume × density) |

and a trace down onto each box names its surface. In Sandbox's zoo the
props' model files name theirs: the office chair is rubber, plastic,
metal, fabric and slippy_wheels; the car seat fabric and metal sheet. All
18 bodies are asleep within two seconds, a player joins and stands, and
the cubes and joints above measure as they did.

```sh
sh tools/compile.sh ../sbox-public/game/core surface
sh tools/surfacerig.sh ../sbox-public /tmp/surfacerig
SBOX=../sbox-public sh spike.sh 20 +shimgame /tmp/surfacerig/.sbproj
grep surfacerig target/spike/server.log
```

## A real game: Facepunch's Sandbox

[Facepunch/sandbox](https://github.com/Facepunch/sandbox) (MIT), s&box's
Garry's Mod: 379 C# files, 151 prefabs, its own resource types. Its code
compiles in the engine, its resources (all JSON but its models and
materials) load, it hosts, and a player joins it.

Its map (`facepunch.flatgrass`) and some of its content
(`facepunch.sbox_content`, graffiti) are packages on sbox.game under
Facepunch's terms, not in the repository, and aren't fetched here. So
`tools/sandbox.sh` starts its dedicated server in one of its own scenes,
`zoo.scene` (a test level of editor meshes, props, a jointed car), and
adds a watcher. Measured (`grep shimwatch target/spike/server.log`):

- The level's 11 editor meshes have collision. Their collision is built
  at runtime by managed's `ModelBuilder` through the native model builder
  (`engine2/src/models.rs`: models, their bodies' descriptions, hulls,
  meshes and the physics container managed reads them back from, and the
  polygon triangulation the editor mesh is made with).
- All 18 of its bodies come to rest on them and sleep within two
  seconds: the car (a hull on four hinged wheels), and props whose
  collision is their model file's (`ModelCollider`, below).
- A player joins, spawns, and stands on the floor (`on Block`): the
  player controller's ground traces run on this library's physics.

```sh
sh tools/compile.sh ../sbox-public/game/core surface prefab sound decal
git clone https://github.com/Facepunch/sandbox ../sandbox
sh tools/sandbox.sh ../sandbox
SBOX=../sbox-public sh spike.sh 120 +shimgame $PWD/../sandbox/sandbox.sbproj +shimhost
# once "Server started!" (startup waits a while on sbox.game for what it can't have):
(cd ../sbox-public/game && ./sbox-server +shimclient 127.0.0.1:27015 +instanceid 2)
grep shimwatch target/spike/server.log
```

**The size of a server, measured.** The server and a player together call
540 distinct slots: 244 written by hand, 106 generated (handles and
remembered properties), 190 still stubs, most of them rendering, the
scene's render objects, Steam's bookkeeping and sound.

**Model files** (`engine2/src/assets/`, `fs.rs`). A model is a compiled
Source 2 resource (`.vmdl_c`): a container of blocks, most of them binary
KeyValues 3. Both are read here as ValveResourceFormat (MIT) reads them
(KV3 versions 1 to 5, uncompressed or LZ4). A model's collision is its
`PHYS` block (or the `.vphys_c` its `DATA` names): parts of spheres,
capsules, hulls and triangle meshes, with their masses and the bones they
follow, made into the same objects a model built at runtime is, so
`ModelCollider`, `ModelPhysics` and `Prop` take shapes from it as from
any. The engine's file system finds the file: managed links each file it
knows (a project's, one it downloaded) and names its project folders.
Measured: all 12 of the zoo's model colliders have their collision (a
washing machine of 80 kg among them), and their props rest.

The models it reads are the ones s&box itself downloads from sbox.game
into its cache (`game/download`): Facepunch's and creators' content,
under their terms, read where it lies and never copied into this
repository. The engine's own core models (`models/dev/box.vmdl`...) and
the player's (Citizen) come only with Facepunch's closed download, and
aren't read. `cargo test` checks a downloaded model when
`ENGINE2_SAMPLE_ASSETS=<sbox-public>/game/download/assets` names a cache.

**The rest of a model** (`engine2/src/assets/model.rs`, `anim.rs`): its
skeleton (bones, their parents, their bind pose in their parent's space
and the model's), its meshes (embedded in the model or `.vmesh_c` files
of their own: their bounds, draw calls, LODs and the mesh groups that
make its body groups), the hitbox sets and attachments its meshes carry,
and its sequences and animations, decoded from the model's ANIM, AGRP and
ASEQ blocks (ValveResourceFormat's segment decoders: full, static, delta
and compressed positions, rotations and scales). That is the 70 `CModel_*`,
`CHitBox*` and `CAttachment` slots a game reads a model through, so
`Model.Bones`, `Model.HitboxSet`, `Model.Attachments` and body groups read
as the file says. Every model in the cache reads (185: 656 meshes, 1,227
bones, 1,409 playable sequences).

**A model's key values** (`engine2/src/assets/kv3text.rs`): what its
author set on it, kept as KV3 text in the model (`m_keyValueText`): its
`prop_data` (a prop's health, whether it explodes and how hard) and its
`break_list` (the gibs it breaks into). Read into KV3 values, handed to
managed by name (`FindModelSubKey`) and written as the JSON it
deserializes them from (`KeyValues3ToJson`). All 52 cached models that
have any read (38 with `prop_data`, 87 break pieces). Measured in the zoo:
the TV has 15 health and six gibs, the explosive oil drum 100 and four;
killed, the TV is gone and its six gibs fall and sleep within three
seconds. Set off, the oil drum breaks into its gibs and spawns core's
explosion (`prefabs/engine/explosion_med.prefab`: compile core's
prefabs, sounds and decals too), whose `RadiusDamage` finds its
neighbours, traces its line of sight to each and pushes them for one
step: on the next, an unshadowed gib of 1 kg 128 away goes 11,916 in/s,
the 11,945 its force predicts (1e10/(d²+1) × dt / m); a body behind
another doesn't move.

And a model in the scene has a pose (`engine2/src/animation.rs`, the
`CSceneAnimatableObject` slots `SceneModel` drives): its bones in parent
space, as its animation graph leaves them (below), or the sequence
playing (a SkinnedModelRenderer's `Sequence`, played straight, looping or
not, at its rate, when it doesn't use its graph), or in bind pose,
with the renderer's procedural bones over them; carried to the model's
space, where a ragdoll's physics bones win; and placed in the world by the
object's transform. Bone merge too (clothing takes its body's pose). A
model's bones in the world are what `ModelHitboxes` places its hitboxes
on, in the hitbox system's own physics world, which `UseHitboxes()`
traces. Before this every scene model fell back to the (absent) error
model, which has no meshes.

Measured under the real server (`grep shimwatch target/spike/server.log`,
the watcher `tools/sandbox.sh` adds): the first-person arms s&box
downloads (70 bones, 48 capsule hitboxes, 114 animations) placed in the
zoo with a `SkinnedModelRenderer` and `ModelHitboxes`. A trace at a bone
hits that bone's hitbox (`hit hand_R on shimtarget ... after 198.9`) and
one 300 above it misses; turned a quarter and moved, the hitboxes follow
(the same shot at the left arm now hits the right one, in front of it);
playing `Punching_Right_Hit_01` (0.83s, looping), the hand swings out
from where it rested (9 from its graph's pose, 24 to 30 from its bind
pose), and a shot where it is hits `hand_R`. A player joins and stands as
before.

**Animation graphs** (`engine2/src/animgraph.rs`). A model's graph (the
`.vanmgrph` its key values name, or one managed sets) is read from its
KV3, the editor's own graph: its parameters, which managed lists and sets
(`AnimVariant`s, a trigger resetting itself once seen), and its nodes,
run each step from the root down, each once however many take it:
sequences and single frames, 1D and 2D blends with their inputs damped,
adds and subtracts (the additive layers a `_delta` sequence feeds), bone
masks (the model's weight lists, a bone not named taking its parent's),
weighted random choices, selectors by bool, enum or int, state machines
(states, transitions on parameter, time, finished, cycle and tag
conditions, blended over their duration, resetting what they say), speed
scale and cycle control. IK, look-at and foot locks pass their input
through. Measured on the arms' own punching graph (`cargo test`, from the
download cache): set `b_attack` and it resets itself, a punch the graph
chooses plays (`Punching_Left_Miss_02`, `Punching_Right_Miss_01`, ...),
the hand it's for swings out furthest, and both come back to rest. Under
the real server (the watcher, from tick 40): managed finds the graph (73
parameters), sets a weapon's speeds and `b_attack` through the shim, and
each attack swings a hand 9 to 11 out, left or right as the graph picks.

**Maps** (`engine2/src/world.rs`, `assets/vpk.rs`, `assets/entities.rs`).
A map is a pak (`maps/flatgrass.vpk`, Valve's VPK, read here) of its
world: its collision (`world_physics.vphys`), its entities
(`entities/default_ents.vents_c`) and its game objects (`world.scene`).
`CreateWorld` mounts the pak, so the engine's files find what's in it;
the world answers its folder and its entity lumps, each entity's keys
(Source 2's entity blob: keys by StringToken hash, typed values) as the
text managed converts; the collision is a physics resource managed loads
by name, made into shapes as a model's is, with the collision attributes
(tags) its shapes name. Sandbox's own map, Facepunch's flatgrass, is a
Steam workshop item now, which needs Steam; its last revision on
sbox.game's CDN is fetched with `tools/map.sh` into the project, and
`tools/sandbox.sh` points Sandbox's map at it. Measured: flatgrass's 249
entities (190 spawn points, lights, fog, soundscapes) and its collision
(1,246 hulls and two meshes over 12 surfaces) load; traces from above
land on grass at z 0 and on the spawn building's roof at 382; a player
joins, spawns on the roof and stands there.

A player runs its own copy of the world, and the map's too. A player of
a local project's map gets the game's code from the server but not its
pak: `ENGINE2_CONTENT` names folders it looks in for files (the pak
`tools/map.sh` fetched). Without it a player has no collision and falls,
as the server sees.

```sh
sh tools/map.sh facepunch.flatgrass ../sandbox/Assets
sh tools/sandbox.sh ../sandbox scenes/sandbox.scene
SBOX=../sbox-public sh spike.sh 300 +shimgame $PWD/../sandbox/sandbox.sbproj +shimhost
# once "Server started!":
(cd ../sbox-public/game && ENGINE2_CONTENT=$PWD/../../sandbox/Assets ./sbox-server +shimclient 127.0.0.1:27015 +instanceid 2)
```

What the game's components still can't do:

- **Animation graphs' rest.** IK, look-at and foot locks aren't solved
  (their input passes through), root motion isn't taken out, and neither
  animation events nor tag events fire for managed. Nobody has compared
  a pose with Facepunch's engine: a graph runs as its nodes are read
  here.
- **Runtime models' rest.** A model built at runtime (`ModelBuilder`)
  keeps its collision, not its bones, hitboxes and meshes.
- **Rolling resistance.** A surface's is kept but not simulated: Rapier
  has none.
- **Workshop maps.** A map that's only on Steam's workshop needs Steam
  to download; `tools/map.sh` takes the newest revision sbox.game's CDN
  still holds. A player sees each map's spawn points twice (380 to the
  server's 190): not looked into yet.

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

**sbox-public.patch** changes a few things locally, none shipped:

- `System.Speech` comes from NuGet, not the Windows DLL their setup downloads.
- The server's launcher prints every managed log line to stdout (and says
  whether it is a dedicated server), and runs `+shimgame <project.sbproj>`
  through the `game` command 30 frames in. The engine runs `+game` itself,
  but only as a package to download from sbox.game; a local project goes
  through the command. A path there is the server's: give it absolute.
- `+shimhost` runs `host` once the game and its scene have loaded: it
  opens the lobby, as a networked game's `NetworkHelper` does for itself.
- `+shimclient <address>` boots the same launcher as a headless player (a
  game app, not a dedicated server) and runs `connect <address>`.
- A dedicated server running a local project counts as a developer host
  (`NetworkSystem.IsDeveloperHost`). It already sends such a game's code
  to its players (`GameInstanceDll`'s code archives), but told them it was
  no developer host, so they looked for a published package and failed.
  And it names its game `org.ident#local` to them: a local project that
  shares its ident with a published game (Facepunch's own Sandbox) sent
  its players to download the published one, through Steam's workshop.
  These are fixes upstream could take.

`tools/compile.sh` copies a project's scenes and prefabs to their `_c`
names, which is all a JSON resource needs here. A GameResource s&box
compiled (a package's sounds, clothing and prefabs from sbox.game) is a
resource container, read for its `DATA` block, the same JSON; its other
blocks are found by name for managed (a GameResource's binary blob). Models, materials and
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
| `engine2/src/surfaces.rs` | surfaces: what shapes are made of, by index and by name, and what each does to them |
| `engine2/src/physics/joints.rs` | joints: s&box's nine kinds as Rapier's generic joint, and when they break |
| `engine2/src/animgraph.rs` | animation graphs: read from their KV3, and run per scene object |
| `engine2/src/models.rs` | models, built at runtime or read from their files: their bodies, hulls and meshes, the physics container managed reads back, polygon triangulation |
| `engine2/src/assets/` | compiled Source 2 resources: the container (`resource.rs`), binary KeyValues 3 (`kv3.rs`) and as text (`kv3text.rs`), models (`model.rs`) and their animations (`anim.rs`), paks (`vpk.rs`), entity lumps (`entities.rs`) |
| `engine2/src/fs.rs` | the engine's file system: the files managed links, its project folders (case blind, as managed names them), the game's own, mounted paks |
| `engine2/src/world.rs` | maps: worlds from a map's pak, their folder and entity lumps |
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
| `tools/jointrig.sh`, `tools/jointrig/` | five rigs of s&box's joint components, and what each does |
| `tools/map.sh` | a map package's paks from sbox.game, into a project |
| `tools/surfacerig.sh`, `tools/surfacerig/` | balls and boxes of core's surfaces, and how they bounce, slide, weigh and trace |
| `tools/sandbox.sh`, `tools/sandbox/` | Facepunch's Sandbox, made ready to host here, and a watcher of its world |

## Next

1. **A server that runs a game.** Facepunch's Sandbox hosts on its own map
   and is joined (above), and models run their animation graphs. What's
   left is fidelity: IK, root motion, animation events.
2. **Steam's part.** Connections, a server's logon, the Elo Pros sign-in,
   the shard list and the relay are ours now, and a player joins (above).
   Next: lobbies. A relay run for others wants a price, as the stream relay's
   credits do (scry-forge `meter/relay.py`).
3. **Assets.** Models (collision, bones, hitboxes, sequences, key
   values, animation graphs), surfaces and maps are read (above).
   Materials, textures and shaders only matter to a client.
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
