use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde_json::Value;

const PREFIXES: [&str; 4] = ["ev_", "event_kind_", "notify_", "time_in_"];
// ! Read by the desktop shell through the core, whose build does not see its sources.
const SHELL_PREFIX: &str = "tray_";

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let folder = root.join("../messages");
    let sources = root.join("src");
    println!("cargo:rerun-if-changed={}", folder.display());
    println!("cargo:rerun-if-changed={}", sources.display());

    let mut named = BTreeSet::new();
    named_keys(&sources, &mut named);

    let mut files: Vec<PathBuf> = fs::read_dir(&folder)
        .expect("the messages folder sits next to miner-core")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    files.sort();

    let mut out = String::from("pub(crate) static TEXTS: &[(&str, &[(&str, Text)])] = &[\n");

    for path in files {
        println!("cargo:rerun-if-changed={}", path.display());
        let tag = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("a language file named by its tag");
        let raw = fs::read_to_string(&path).expect("a readable language file");
        let messages: serde_json::Map<String, Value> =
            serde_json::from_str(&raw).expect("a language file in JSON");

        if tag == "en" {
            if let Some(key) = named.iter().find(|key| !messages.contains_key(*key)) {
                panic!("the core names {key}, which en.json lacks");
            }
        }

        writeln!(out, "    ({tag:?}, &[").unwrap();

        for (key, value) in &messages {
            if named.contains(key) || key.starts_with(SHELL_PREFIX) {
                writeln!(out, "        ({key:?}, {}),", text(tag, key, value)).unwrap();
            }
        }

        out.push_str("    ]),\n");
    }

    out.push_str("];\n");

    let target = Path::new(&env::var("OUT_DIR").expect("set by cargo")).join("texts.rs");
    fs::write(target, out).expect("OUT_DIR is writable");
}

// ! The dashboard's own texts share these prefixes: only the keys quoted in the core are embedded.
fn named_keys(dir: &Path, keys: &mut BTreeSet<String>) {
    for entry in fs::read_dir(dir).expect("a readable source folder") {
        let path = entry.expect("a readable source entry").path();

        if path.is_dir() {
            named_keys(&path, keys);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let source = fs::read_to_string(&path).expect("a readable source file");

            keys.extend(quoted_keys(&source).map(str::to_owned));
        }
    }
}

fn quoted_keys(source: &str) -> impl Iterator<Item = &str> {
    source.match_indices('"').filter_map(move |(at, _)| {
        let rest = &source[at + 1..];
        let end =
            rest.find(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'))?;
        let key = &rest[..end];

        (rest[end..].starts_with('"') && PREFIXES.iter().any(|prefix| key.starts_with(prefix)))
            .then_some(key)
    })
}

// ! The core reads a plain text or a plural on one number, as the dashboard
// does; any other shape stops the build instead of leaving the text out.
fn text(tag: &str, key: &str, value: &Value) -> String {
    match value.as_str() {
        Some(text) => format!("Text::Plain({text:?})"),
        None => plural(value).unwrap_or_else(|| {
            panic!("{tag}.json: {key} is neither a text nor a plural on one number")
        }),
    }
}

fn plural(value: &Value) -> Option<String> {
    let [message] = value.as_array()?.as_slice() else {
        return None;
    };
    let [selector] = message["selectors"].as_array()?.as_slice() else {
        return None;
    };
    let selector = selector.as_str()?;
    let input = message["declarations"]
        .as_array()?
        .iter()
        .filter_map(Value::as_str)
        .find_map(|declaration| {
            let (name, plural) = declaration.strip_prefix("local ")?.split_once('=')?;
            let input = plural.trim().strip_suffix(": plural")?;

            (name.trim() == selector).then(|| input.trim().to_owned())
        })?;
    let variants = message["match"]
        .as_object()?
        .iter()
        .map(|(case, text)| {
            let category = case.strip_prefix(selector)?.strip_prefix('=')?;

            Some(format!("({category:?}, {:?})", text.as_str()?))
        })
        .collect::<Option<Vec<String>>>()?;

    Some(format!(
        "Text::Plural {{ input: {input:?}, variants: &[{}] }}",
        variants.join(", ")
    ))
}
