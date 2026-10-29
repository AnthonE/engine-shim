//! KeyValues 3 as text, the form a model keeps its key values in
//! (`m_keyValueText`: its `prop_data`, its `break_list` of gibs...), read
//! into the same values binary KV3 gives; and any of them written as the
//! JSON managed deserializes them from (`KeyValues3ToJson`).
//!
//! Text KV3: a header comment, then an object of `key = value`; arrays in
//! brackets, commas between; strings quoted (or triple-quoted, across
//! lines); numbers, true, false, null; a value may carry a flag
//! (`resource:"models/x.vmdl"`), and bytes come as `#[ 0a 1b ... ]`.
use super::kv3::Value;

pub fn read(text: &str) -> Result<Value, String> {
    let mut p = Parser { s: text.as_bytes(), at: 0 };
    p.space()?;
    let v = p.value()?;
    p.space()?;
    if p.at < p.s.len() { return Err(p.error("text after the value")); }
    Ok(v)
}

struct Parser<'a> {
    s: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn error(&self, what: &str) -> String {
        format!("kv3 text: {what} at byte {}", self.at)
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.at).copied()
    }

    fn starts(&self, with: &str) -> bool {
        self.s[self.at..].starts_with(with.as_bytes())
    }

    /// Past whitespace and comments (the header is one).
    fn space(&mut self) -> Result<(), String> {
        loop {
            while self.peek().is_some_and(|c| c.is_ascii_whitespace()) { self.at += 1; }
            let close = if self.starts("<!--") { "-->" } else if self.starts("/*") { "*/" } else if self.starts("//") { "\n" } else { return Ok(()) };
            self.at += 2;
            match self.s[self.at..].windows(close.len()).position(|w| w == close.as_bytes()) {
                Some(n) => self.at += n + close.len(),
                None if close == "\n" => self.at = self.s.len(),
                None => return Err(self.error("an unclosed comment")),
            }
        }
    }

    fn word(&mut self) -> &str {
        let start = self.at;
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.' | b'-' | b'+')) { self.at += 1; }
        std::str::from_utf8(&self.s[start..self.at]).unwrap_or("")
    }

    fn value(&mut self) -> Result<Value, String> {
        match self.peek() {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string().map(Value::String),
            Some(b'#') if self.starts("#[") => self.blob(),
            Some(_) => {
                let start = self.at;
                let w = self.word().to_string();
                if w.is_empty() { return Err(self.error("no value")); }
                // a flag before the value it marks (resource:"...", subclass:{...}): the value is what counts
                if self.peek() == Some(b':') {
                    self.at += 1;
                    return self.value();
                }
                Ok(match w.as_str() {
                    "true" => Value::Bool(true),
                    "false" => Value::Bool(false),
                    "null" => Value::Null,
                    _ if w.contains(['.', 'e', 'E']) && !w.starts_with("0x") || matches!(w.as_str(), "inf" | "-inf" | "nan") => {
                        Value::Double(w.parse().map_err(|_| { self.at = start; self.error("a bad number") })?)
                    }
                    _ => match w.parse::<i64>() {
                        Ok(i) => Value::Int(i),
                        Err(_) => Value::UInt(w.parse().map_err(|_| { self.at = start; self.error("a bad number") })?),
                    },
                })
            }
            None => Err(self.error("the end, wanting a value")),
        }
    }

    fn object(&mut self) -> Result<Value, String> {
        self.at += 1;
        let mut members = Vec::new();
        loop {
            self.space()?;
            match self.peek() {
                Some(b'}') => { self.at += 1; return Ok(Value::Object(members)); }
                None => return Err(self.error("an unclosed object")),
                _ => {}
            }
            let key = if self.peek() == Some(b'"') { self.string()? } else { self.word().to_string() };
            if key.is_empty() { return Err(self.error("no key")); }
            self.space()?;
            if self.peek() != Some(b'=') { return Err(self.error("no = after a key")); }
            self.at += 1;
            self.space()?;
            members.push((key, self.value()?));
        }
    }

    fn array(&mut self) -> Result<Value, String> {
        self.at += 1;
        let mut items = Vec::new();
        loop {
            self.space()?;
            match self.peek() {
                Some(b']') => { self.at += 1; return Ok(Value::Array(items)); }
                Some(b',') => { self.at += 1; continue; }
                None => return Err(self.error("an unclosed array")),
                _ => items.push(self.value()?),
            }
        }
    }

    fn string(&mut self) -> Result<String, String> {
        if self.starts("\"\"\"") {
            self.at += 3;
            let n = self.s[self.at..].windows(3).position(|w| w == b"\"\"\"").ok_or_else(|| self.error("an unclosed string"))?;
            let s = String::from_utf8_lossy(&self.s[self.at..self.at + n]).into_owned();
            self.at += n + 3;
            // the lines between the quotes, not the line breaks after and before them
            return Ok(s.strip_prefix('\n').map(|s| s.strip_suffix('\n').unwrap_or(s)).unwrap_or(&s).to_string());
        }
        self.at += 1;
        let mut out = Vec::new();
        loop {
            match self.peek() {
                None => return Err(self.error("an unclosed string")),
                Some(b'"') => { self.at += 1; break; }
                Some(b'\\') => {
                    self.at += 1;
                    out.push(match self.peek() {
                        Some(b'n') => b'\n',
                        Some(b't') => b'\t',
                        Some(c) => c,
                        None => return Err(self.error("an unclosed string")),
                    });
                    self.at += 1;
                }
                Some(c) => { out.push(c); self.at += 1; }
            }
        }
        Ok(String::from_utf8_lossy(&out).into_owned())
    }

    fn blob(&mut self) -> Result<Value, String> {
        self.at += 2;
        let mut bytes = Vec::new();
        loop {
            self.space()?;
            match self.peek() {
                Some(b']') => { self.at += 1; return Ok(Value::Blob(bytes)); }
                None => return Err(self.error("unclosed bytes")),
                _ => {
                    let w = self.word();
                    let b = u8::from_str_radix(w, 16).map_err(|_| self.error("a bad byte"))?;
                    bytes.push(b);
                }
            }
        }
    }
}

/// A value as JSON: objects and arrays as they are, flags gone, bytes as
/// an array of numbers, numbers that aren't any (NaN, infinities) null.
pub fn json(v: &Value, out: &mut String) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Int(i) => out.push_str(&i.to_string()),
        Value::UInt(u) => out.push_str(&u.to_string()),
        Value::Double(d) if d.is_finite() => out.push_str(&d.to_string()),
        Value::Double(_) => out.push_str("null"),
        Value::String(s) => string(s, out),
        Value::Blob(b) => {
            out.push('[');
            out.push_str(&b.iter().map(u8::to_string).collect::<Vec<_>>().join(","));
            out.push(']');
        }
        Value::Array(a) => {
            out.push('[');
            for (i, x) in a.iter().enumerate() {
                if i > 0 { out.push(','); }
                json(x, out);
            }
            out.push(']');
        }
        Value::Object(m) => {
            out.push('{');
            for (i, (k, x)) in m.iter().enumerate() {
                if i > 0 { out.push(','); }
                string(k, out);
                out.push(':');
                json(x, out);
            }
            out.push('}');
        }
    }
}

fn string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_models_key_values_read_and_write_as_json() {
        // as a model keeps them: a gas cylinder's, in part
        let text = "<!-- kv3 encoding:text:version{e21c7f3c-8a33-41c5-9977-a76d3a32aa0d} format:vmdlkeys3:version{141d1e66-eebf-4440-be28-a74cd93426eb} -->\n{\n\tprop_data = \n\t{\n\t\tbakelighting = true\n\t\thealth = 60.0\n\t\texplosive_radius = 250.0\n\t}\n\tbreak_list = \n\t[\n\t\t{\n\t\t\tpiece_name = \"\"\n\t\t\tmodel = resource:\"models/props/gas_cylinder/gas_cylinder_gib_01.vmdl\"\n\t\t\toffset = [ 0.0, -6.0, 19.0 ]\n\t\t},\n\t]\n}";
        let v = read(text).unwrap();
        let prop = v.get("prop_data").unwrap();
        assert_eq!(prop.get("health"), Some(&Value::Double(60.0)));
        assert_eq!(prop.get("bakelighting"), Some(&Value::Bool(true)));
        let mut s = String::new();
        json(v.get("break_list").unwrap(), &mut s);
        assert_eq!(s, r#"[{"piece_name":"","model":"models/props/gas_cylinder/gas_cylinder_gib_01.vmdl","offset":[0,-6,19]}]"#);
        let mut s = String::new();
        json(prop, &mut s);
        assert_eq!(s, r#"{"bakelighting":true,"health":60,"explosive_radius":250}"#);
    }

    #[test]
    fn the_rest_of_text_kv3() {
        let v = read("// a comment\n{ /* and another */ \"quoted key\" = \"a \\\"quote\\\"\\n\" big = 18446744073709551615 n = -3\n  lines = \"\"\"\none\ntwo\n\"\"\" bytes = #[ 0a ff 00 ] nothing = null sub = subclass:{ x = 1e2 } }").unwrap();
        assert_eq!(v.get("quoted key"), Some(&Value::String("a \"quote\"\n".into())));
        assert_eq!(v.get("big"), Some(&Value::UInt(u64::MAX)));
        assert_eq!(v.get("n"), Some(&Value::Int(-3)));
        assert_eq!(v.get("lines"), Some(&Value::String("one\ntwo".into())));
        assert_eq!(v.get("bytes"), Some(&Value::Blob(vec![10, 255, 0])));
        assert_eq!(v.get("nothing"), Some(&Value::Null));
        assert_eq!(v.get("sub").and_then(|s| s.get("x")), Some(&Value::Double(100.0)));
        let mut s = String::new();
        json(&v, &mut s);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&s).unwrap()["quoted key"], "a \"quote\"\n");
        assert!(read("{ a = ").is_err());
        assert!(read("{ a = 1 } junk").is_err());
    }
}
