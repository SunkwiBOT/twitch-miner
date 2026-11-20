use std::path::Path;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use rand::RngCore;

// ! A panic while the lock was held leaves plain data behind: it is used as is.
pub fn locked<T: ?Sized>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

pub fn read_locked<T: ?Sized>(lock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    lock.read().unwrap_or_else(PoisonError::into_inner)
}

pub fn write_locked<T: ?Sized>(lock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    lock.write().unwrap_or_else(PoisonError::into_inner)
}

pub fn random_hex(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    rand::rng().fill_bytes(&mut buffer);
    hex::encode(buffer)
}

pub fn random_device_id() -> String {
    random_hex(16)
}

// ! RFC 4122 version 4: the version/variant nibbles are fixed, the rest random.
pub fn random_uuid_v4() -> String {
    let mut bytes = [0u8; 16];

    rand::rng().fill_bytes(&mut bytes);
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    let hex = hex::encode(bytes);

    format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    )
}

// ! A resolved IANA zone name, accepted only if `chrono_tz` knows it (its
// data is embedded, so this works the same on a `FROM scratch` image with no
// system tzdata). The `AUTO` sentinel used by `GlobalSettings::time_zone` is
// a display-only preference and is never valid here.
pub fn parse_time_zone(time_zone: &str) -> Option<chrono_tz::Tz> {
    chrono_tz::Tz::from_str(time_zone).ok()
}

// ! Minutes since local midnight, in the given IANA time zone. `time_zone`
// is expected to already be validated (see `parse_time_zone`); an unparsable
// value falls back to UTC rather than the host's own zone, which on the
// published Docker image (no tzdata) would be UTC anyway and on desktop
// would silently disagree with whatever zone the caller meant to use.
pub fn local_minutes_of_day(now: chrono::DateTime<chrono::Utc>, time_zone: &str) -> u16 {
    use chrono::Timelike;

    let local = match parse_time_zone(time_zone) {
        Some(zone) => now.with_timezone(&zone).naive_local(),
        None => now.naive_utc(),
    };

    (local.hour() * 60 + local.minute()) as u16
}

// ! The process's own zone, from the `TZ` env var or the system tzdata:
// what `AUTO` (the display-only sentinel for `GlobalSettings::time_zone`)
// resolves to. This is what makes the published Docker image's `TZ=`
// reach exported reports.
fn auto_time_zone() -> chrono_tz::Tz {
    std::env::var("TZ")
        .ok()
        .and_then(|name| parse_time_zone(&name))
        .or_else(|| {
            iana_time_zone::get_timezone()
                .ok()
                .and_then(|name| parse_time_zone(&name))
        })
        .unwrap_or(chrono_tz::Tz::UTC)
}

// ! The zone exported reports are formatted in: `AUTO` resolves through
// the process's own zone, any other value is parsed as an IANA name,
// falling back to UTC when unparsable (the display preference is never
// validated as strictly as `GlobalSettings::quiet_window.time_zone`).
pub fn display_time_zone(time_zone: &str) -> chrono_tz::Tz {
    if time_zone == crate::config::AUTO_TIME_ZONE {
        auto_time_zone()
    } else {
        parse_time_zone(time_zone).unwrap_or(chrono_tz::Tz::UTC)
    }
}

// ! Carries the UTC offset so exported lines stay unambiguous across a
// daylight-saving change.
pub fn format_local(at: chrono::DateTime<chrono::Utc>, zone: chrono_tz::Tz) -> String {
    at.with_timezone(&zone)
        .format("%Y-%m-%d %H:%M:%S %:z")
        .to_string()
}

// ! A folder another user owns (a mount shared on purpose) keeps its mode: only its owner may change it.
pub fn private_dir(dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700));
    }

    Ok(())
}

// ! Twitch's own pages share the channel URL shape: a login is never one of these.
const TWITCH_PAGES: [&str; 14] = [
    "videos",
    "directory",
    "settings",
    "search",
    "downloads",
    "drops",
    "inventory",
    "wallet",
    "subscriptions",
    "friends",
    "messages",
    "prime",
    "turbo",
    "activate",
];

pub fn normalize_login(input: &str) -> Option<String> {
    let text = input.trim().to_ascii_lowercase();

    let login = match text.find("twitch.tv/") {
        Some(at) => {
            let mut parts = text[at + "twitch.tv/".len()..]
                .split(['/', '?', '#'])
                .filter(|part| !part.is_empty());
            match parts.next() {
                Some("popout" | "moderator") => parts.next().unwrap_or(""),
                Some(page) if TWITCH_PAGES.contains(&page) => "",
                Some(part) => part,
                None => "",
            }
        }
        None => text
            .trim_start_matches('@')
            .split('?')
            .next()
            .unwrap_or("")
            .trim_end_matches('/'),
    };

    let trimmed = login.to_owned();

    if trimmed.is_empty()
        || trimmed.len() > 25
        || !trimmed
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    {
        return None;
    }

    Some(trimmed)
}

pub fn game_slug(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    let mut dash = false;

    for ch in name.chars() {
        // ! Twitch drops apostrophes ("Assassin's Creed" is assassins-creed)
        // and writes accented letters plainly ("Pokémon" is pokemon).
        if matches!(ch, '\'' | '\u{2019}') {
            continue;
        }

        let ch = fold_accent(ch);

        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash && !slug.is_empty() {
            slug.push('-');
            dash = true;
        }
    }

    slug.trim_end_matches('-').to_owned()
}

fn fold_accent(ch: char) -> char {
    match ch {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => 'a',
        'ç' | 'Ç' => 'c',
        'è' | 'é' | 'ê' | 'ë' | 'È' | 'É' | 'Ê' | 'Ë' => 'e',
        'ì' | 'í' | 'î' | 'ï' | 'Ì' | 'Í' | 'Î' | 'Ï' => 'i',
        'ñ' | 'Ñ' => 'n',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => 'o',
        'ù' | 'ú' | 'û' | 'ü' | 'Ù' | 'Ú' | 'Û' | 'Ü' => 'u',
        'ý' | 'ÿ' | 'Ý' => 'y',
        _ => ch,
    }
}

pub async fn sleep_until_some(at: Option<tokio::time::Instant>) {
    match at {
        Some(at) => tokio::time::sleep_until(at).await,
        None => std::future::pending().await,
    }
}
