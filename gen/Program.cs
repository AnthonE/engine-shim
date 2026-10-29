// Writes engine2/src/slots.rs: one Rust stub for every slot of an igen_*
// table, typed exactly as the managed side calls it, so a stub that returns
// a struct by value or takes one keeps the C ABI the caller expects.
//
//   dotnet run --project gen -- <sbox-public> <ident> <out.rs>
//
// The slot order comes from the generated managed interop file
// (engine/Sandbox.Engine/Interop.<Ident>.cs: "X.__N.Name = (delegate*
// ...) nativeFunctions[i]"); the types come from the built assembly by
// reflection, which is where the managed side's layouts are decided.
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.RegularExpressions;

var root = args[0];
var ident = args[1];
var outPath = args[2];
var managed = Path.Combine(root, "game", "bin", "managed");
var csFile = Directory.GetFiles(Path.Combine(root, "engine"), $"Interop.{char.ToUpper(ident[0])}{ident[1..]}.cs", SearchOption.AllDirectories)
    .Single(f => !f.Contains("/obj/"));
AppDomain.CurrentDomain.AssemblyResolve += (_, e) =>
{
    var p = Path.Combine(managed, new AssemblyName(e.Name).Name + ".dll");
    return File.Exists(p) ? Assembly.LoadFrom(p) : null;
};
var asmName = ident == "engine" ? "Sandbox.Engine" : $"Sandbox.{char.ToUpper(ident[0])}{ident[1..]}";
var asm = Assembly.LoadFrom(Path.Combine(managed, asmName + ".dll"));

// slot -> (managed field path)
var slotRe = new Regex(@"^\s*([\w\.]+)\.(\w+) = \(delegate\* unmanaged[^<]*<[^;]*?>\)\s*\(?\s*nativeFunctions\[(\d+)\]", RegexOptions.Multiline);
var text = File.ReadAllText(csFile);
var slots = new SortedDictionary<int, (string type, string field)>();
foreach (Match m in slotRe.Matches(text)) slots[int.Parse(m.Groups[3].Value)] = (m.Groups[1].Value, m.Groups[2].Value);
var total = int.Parse(Regex.Match(text, @"nativeFunctions = new IntPtr\[(\d+)\]").Groups[1].Value);
var managedCount = int.Parse(Regex.Match(text, @"managedFunctions = new IntPtr\[(\d+)\]").Groups[1].Value);
var hash = int.Parse(Regex.Match(text, @"nativeInit\( (\d+),").Groups[1].Value);

var structs = new Dictionary<Type, string>();
var structDefs = new StringBuilder();
var warnings = new List<string>();
int SizeOf(Type t) => (int)typeof(Unsafe).GetMethod("SizeOf").MakeGenericMethod(t).Invoke(null, null);

string Rust(Type t)
{
    if (t == typeof(void)) return "()";
    if (t.IsPointer || t.IsByRef || t.IsFunctionPointer || t == typeof(IntPtr) || t == typeof(UIntPtr)) return "usize";
    if (t.IsEnum) t = Enum.GetUnderlyingType(t);
    if (t == typeof(int)) return "i32";
    if (t == typeof(uint)) return "u32";
    if (t == typeof(long)) return "i64";
    if (t == typeof(ulong)) return "u64";
    if (t == typeof(short)) return "i16";
    if (t == typeof(ushort)) return "u16";
    if (t == typeof(char)) return "u16";
    if (t == typeof(byte)) return "u8";
    if (t == typeof(sbyte)) return "i8";
    if (t == typeof(bool)) return "u8";
    if (t == typeof(float)) return "f32";
    if (t == typeof(double)) return "f64";
    if (!t.IsValueType) throw new Exception($"a reference type across the boundary: {t}");
    return Struct(t);
}

// A struct as #[repr(C)] with the managed layout, the one Facepunch's C++
// checks (`sizeof(T)` in C#: a bool is a byte): sequential structs in
// declaration order with repr(C)'s alignment, packed as the managed Pack;
// explicit ones at their FieldOffsets. The size is asserted against the
// managed sizeof.
string Struct(Type t)
{
    if (structs.TryGetValue(t, out var name)) return name;
    name = "S_" + Regex.Replace(t.FullName, @"[^A-Za-z0-9]", "_");
    structs[t] = name;
    var size = SizeOf(t);
    var layout = t.StructLayoutAttribute;
    var pack = layout?.Pack ?? 0;
    var explicitLayout = layout?.Value == LayoutKind.Explicit;
    var fields = t.GetFields(BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic).ToList();
    var sb = new StringBuilder();
    sb.AppendLine($"/// {t.FullName}, {size} bytes{(pack > 0 ? $", Pack {pack}" : "")}{(explicitLayout ? ", explicit" : "")}");
    var repr = explicitLayout ? "C, packed" : pack > 0 && pack < 8 ? $"C, packed({pack})" : "C";
    sb.AppendLine($"#[repr({repr})] #[derive(Clone, Copy)]");
    sb.AppendLine($"pub struct {name} {{");
    int pad = 0;
    string FieldType(FieldInfo f, out int bytes)
    {
        var fixedBuf = f.GetCustomAttribute<FixedBufferAttribute>();
        if (fixedBuf != null) { bytes = SizeOf(fixedBuf.ElementType) * fixedBuf.Length; return $"[{Rust(fixedBuf.ElementType)}; {fixedBuf.Length}]"; }
        var ft = f.FieldType;
        bytes = ft.IsPointer || ft == typeof(IntPtr) || ft == typeof(UIntPtr) ? 8 : SizeOf(ft.IsEnum ? Enum.GetUnderlyingType(ft) : ft);
        return Rust(ft);
    }
    var overlap = false;
    if (explicitLayout)
    {
        int at = 0;
        foreach (var f in fields.OrderBy(f => f.GetCustomAttribute<FieldOffsetAttribute>().Value))
        {
            var off = f.GetCustomAttribute<FieldOffsetAttribute>().Value;
            if (off < at) { overlap = true; break; }
            if (off > at) sb.AppendLine($"    _pad{pad++}: [u8; {off - at}],");
            sb.AppendLine($"    pub {Field(f.Name)}: {FieldType(f, out var n)},");
            at = off + n;
        }
        if (!overlap && size > at) sb.AppendLine($"    _pad{pad++}: [u8; {size - at}],");
    }
    else
    {
        foreach (var f in fields) sb.AppendLine($"    pub {Field(f.Name)}: {FieldType(f, out _)},");
        if (layout?.Size > 0) warnings.Add($"{t.FullName}: Size = {layout.Size}, checked by the assert");
    }
    if (overlap)
    {
        warnings.Add($"{t.FullName}: fields overlap, carried as raw bytes");
        sb.Clear();
        sb.AppendLine($"/// {t.FullName}, {size} bytes (a union: raw bytes)");
        sb.AppendLine("#[repr(C)] #[derive(Clone, Copy)]");
        sb.AppendLine($"pub struct {name} {{ pub bytes: [u8; {size}] }}");
    }
    else sb.AppendLine("}");
    sb.AppendLine($"const _: () = assert!(core::mem::size_of::<{name}>() == {size});");
    structDefs.AppendLine(sb.ToString());
    return name;
}
static string Field(string n) => "f_" + Regex.Replace(n, @"[^A-Za-z0-9_]", "_");

// slots implemented by hand: engine2/src/impls.rs's `pub unsafe fn <slot name>`
var implsPath = Path.Combine(Path.GetDirectoryName(Path.GetFullPath(outPath)), "impls.rs");
var impls = File.Exists(implsPath)
    ? Regex.Matches(File.ReadAllText(implsPath), @"pub (?:unsafe )?fn (\w+)\s*\(").Select(m => m.Groups[1].Value).ToHashSet()
    : new HashSet<string>();
var implemented = 0;
var stubs = new StringBuilder();
var names = new StringBuilder();
var table = new StringBuilder();
for (int i = 0; i < total; i++)
{
    if (i == 0)
    {
        names.AppendLine("    \"Debug_Error\",");
        table.AppendLine("    debug_error as *const () as usize,");
        continue;
    }
    if (!slots.TryGetValue(i, out var s)) throw new Exception($"no managed field for slot {i}");
    var type = asm.GetType(s.type.Replace(".__N", "+__N")) ?? throw new Exception($"no type {s.type}");
    var field = type.GetField(s.field, BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic) ?? throw new Exception($"no field {s.type}.{s.field}");
    var fp = field.FieldType;
    if (!fp.IsFunctionPointer) fp = field.GetModifiedFieldType();
    var ps = fp.GetFunctionPointerParameterTypes().Select(p => p.UnderlyingSystemType).ToArray();
    var ret = fp.GetFunctionPointerReturnType().UnderlyingSystemType;
    var argList = string.Join(", ", ps.Select((p, k) => $"_a{k}: {Rust(p)}"));
    var r = Rust(ret);
    // a pointer back is an object of its own (zeroed memory: an empty string if read as one); anything else, zeroes
    var body = r == "()" ? "" : r == "usize" ? $"crate::trace::object({i})" : "unsafe { core::mem::zeroed() }";
    // a handle the stub engine made is valid: "is it valid" answers yes
    if (r == "i32" && Regex.IsMatch(s.field, @"(IsValid|IsStrongHandleValid|IsLoaded|IsInitialized)$")) body = "1";
    if (impls.Contains(s.field)) { body = $"unsafe {{ crate::impls::{s.field}({string.Join(", ", ps.Select((_, k) => $"_a{k}"))}) }}"; implemented++; }
    stubs.AppendLine($"unsafe extern \"C\" fn s{i}({argList}){(r == "()" ? "" : $" -> {r}")} {{ hit({i}); {body} }}");
    names.AppendLine($"    \"{s.field}\",");
    table.AppendLine($"    s{i} as *const () as usize,");
}

var o = new StringBuilder();
o.AppendLine($"// Written by gen/ from s&box's managed interop ({Path.GetFileName(csFile)}): do not edit.");
o.AppendLine($"// igen_{ident}: {total} native slots, {managedCount} managed callbacks, hash {hash}.");
o.AppendLine("#![allow(non_camel_case_types, dead_code, unused_unsafe, clippy::all)]");
o.AppendLine("use crate::trace::hit;");
o.AppendLine("use crate::trace::debug_error;");
o.AppendLine($"pub const HASH: i32 = {hash};");
o.AppendLine($"pub const MANAGED: usize = {managedCount};");
o.AppendLine($"pub const SLOTS: usize = {total};");
o.AppendLine();
o.Append(structDefs);
o.Append(stubs);
o.AppendLine();
o.AppendLine($"pub static NAMES: [&str; {total}] = [");
o.Append(names);
o.AppendLine("];");
o.AppendLine();
o.AppendLine($"pub fn table() -> [usize; {total}] {{ [");
o.Append(table);
o.AppendLine("] }");
File.WriteAllText(outPath, o.ToString());
Console.WriteLine($"igen_{ident}: {total} slots ({implemented} implemented in impls.rs, the rest stubs), {structs.Count} struct types, hash {hash} -> {outPath}");
foreach (var w in warnings) Console.WriteLine("  " + w);
