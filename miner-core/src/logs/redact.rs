use std::borrow::Cow;
use std::io::{self, Write};
use std::sync::LazyLock;

use regex::{Regex, RegexSet};
use tracing_subscriber::fmt::MakeWriter;

pub struct Redacted<M>(pub M);

impl<'a, M: MakeWriter<'a>> MakeWriter<'a> for Redacted<M> {
    type Writer = RedactedLine<M::Writer>;

    fn make_writer(&'a self) -> Self::Writer {
        RedactedLine {
            inner: self.0.make_writer(),
            buffer: Vec::new(),
        }
    }
}

pub struct RedactedLine<W: Write> {
    inner: W,
    buffer: Vec<u8>,
}

impl<W: Write> RedactedLine<W> {
    fn emit(&mut self) -> io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let line = String::from_utf8_lossy(&self.buffer);
        let written = self.inner.write_all(redact(&line).as_bytes());

        self.buffer.clear();
        written
    }
}

impl<W: Write> Write for RedactedLine<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.emit()?;
        self.inner.flush()
    }
}

impl<W: Write> Drop for RedactedLine<W> {
    fn drop(&mut self) {
        let _ = self.emit();
    }
}

const SECRETS: [(&str, &str); 7] = [
    (r"(?i)(oauth|bearer)\s+[a-z0-9]{20,}", "$1 [redacted]"),
    (
        r"(?i)(token|sig|signature|auth_token|access_token|refresh_token|device_code)=([^&\s\x22']+)",
        "$1=[redacted]",
    ),
    (
        r"(?i)\x22(access_token|refresh_token|auth_token|device_code|token|sig)\x22\s*:\s*\x22[^\x22]*\x22",
        "\"$1\":\"[redacted]\"",
    ),
    (
        r"(?i)(socks4a?|socks5h?|https?)://([^:/@\s]*):(\S+)@",
        "$1://$2:[redacted]@",
    ),
    (
        r"(?i)(discord(?:app)?\.com/api/webhooks/)[0-9]+/[A-Za-z0-9_-]+",
        "$1[redacted]",
    ),
    (r"\bbot[0-9]+:[A-Za-z0-9_-]+", "bot[redacted]"),
    (r"\b[A-Za-z0-9]{30,}\b", "[redacted]"),
];

const LONG_WORD: usize = SECRETS.len() - 1;

static PATTERNS: LazyLock<(RegexSet, Vec<Regex>)> = LazyLock::new(|| {
    let sources = SECRETS.map(|(pattern, _)| pattern);

    (
        RegexSet::new(sources).unwrap(),
        sources.iter().map(|p| Regex::new(p).unwrap()).collect(),
    )
});

// ! GQL operation names run past 30 letters too ("VideoPlayerStreamInfoOverlayChannel"): a CamelCase word is no secret.
fn camel_case(word: &str) -> bool {
    word.starts_with(|c: char| c.is_ascii_uppercase())
        && word
            .split(|c: char| c.is_ascii_uppercase())
            .skip(1)
            .all(|part| part.len() >= 2 && part.bytes().all(|b| b.is_ascii_lowercase()))
}

pub fn redact(text: &str) -> Cow<'_, str> {
    let (set, patterns) = &*PATTERNS;
    let mut out = Cow::Borrowed(text);

    for index in set.matches(text).iter() {
        let replaced = if index == LONG_WORD {
            patterns[index].replace_all(&out, |found: &regex::Captures| {
                if camel_case(&found[0]) {
                    found[0].to_owned()
                } else {
                    "[redacted]".to_owned()
                }
            })
        } else {
            patterns[index].replace_all(&out, SECRETS[index].1)
        };

        if let Cow::Owned(changed) = replaced {
            out = Cow::Owned(changed);
        }
    }

    out
}
