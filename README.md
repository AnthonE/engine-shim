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

What the game's components still can't do:

- **Native objects.** Lights, models and text renderers fail `Assert:
  IsValid`. A native object made inside a handle scope must call managed's
  `HandleIndex.RegisterHandle` (a managed callback) so managed binds it
  ("NextHandleScope: Didn't use nextObject"). One protocol, every class.
- **Physics.** No physics world is made (`IPhysicsWorld was null when
  calling SetGravity`), so colliders and rigidbodies fail. About 300 slots;
  Rapier or Jolt behind them.

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
SBOX=../sbox-public sh spike.sh 90 +shimgame ../sbox-public/game/samples/sweeper/.sbproj
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
  through the command.

`tools/compile.sh` copies a project's scenes and prefabs to their `_c`
names, which is all a JSON resource needs here. Models, materials and
textures are Source 2's binary formats and wait for the assets step.

No EULA binary is used: the checkout's `game/bin` holds only text files,
and Facepunch's artifact download (`Setup.sh`) is never run.

## Layout

| | |
| --- | --- |
| `gen/` | C#: writes `engine2/src/slots.rs` from the managed interop (the slot order) and the built `Sandbox.Engine.dll` by reflection (every argument and return type, every struct's layout, its size asserted against the managed `sizeof` that Facepunch's C++ checks) |
| `engine2/src/lib.rs` | `igen_engine`: fills the table |
| `engine2/src/slots.rs` | generated: 3,578 typed stubs, 81 struct layouts, the names |
| `engine2/src/impls.rs` | the slots written by hand; a `pub unsafe fn` named as a slot replaces its stub |
| `engine2/src/trace.rs` | which slots are called, in order, and how often |
| `spike.sh` | one turn of the loop |
| `tools/compile.sh` | a project's JSON resources (scenes, prefabs) as compiled resources |

## Next

1. **A server that runs a game.** The handle protocol, then physics, then
   a real open-source s&box game: its trace is the measured size of "a
   server", the slots one game needs.
2. **Steam's part.** A server that is not on Steam needs identity,
   connections and lobbies from somewhere else: the Elo Pros wallet sign-in
   and our relay, behind the `steam_api` functions the managed side calls.
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
