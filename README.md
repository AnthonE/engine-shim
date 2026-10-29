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
on Linux from the public repo and NuGet alone. Run on `libengine2.so` from
this repository, it boots, runs its frame loop at 60 Hz and shuts down in
order. Of the 3,578 slots, 118 are called on the way. 13 are written by
hand (`engine2/src/impls.rs`); every other slot is a generated stub that
records its first call and returns zeroes, or a fresh zeroed block where it
returns a pointer.

It does not load a game yet. Loading Facepunch's Sweeper sample stops at
two static constructors: audio's mixing thread (an assert) and the shader
compiler (it loads `vfx_vulkan`). The game command is registered by the
game instance after both, so it is not there yet.

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
cd ../engine-shim
SBOX=../sbox-public sh spike.sh 40 +game ../sbox-public/game/samples/sweeper/.sbproj
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
- The server's launcher prints every managed log line to stdout, and runs
  `+game` from the command line 30 frames in. Source's command buffer did
  this natively. Doing it in `engine2` is on the list.

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

## Next

1. **A server that loads a game.** Answer the mixing thread and the shader
   compiler as a server would (no audio device, no shaders), run `+game` in
   `engine2`, and load Sweeper. Then a real open-source s&box game, and its
   trace is the measured size of "a server": the slots one game needs.
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
