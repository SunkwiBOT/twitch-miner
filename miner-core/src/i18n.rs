use crate::events::{BetResult, EventKind, MinerEvent};
use crate::scheduler::PointsReason;
use chrono::{DateTime, Utc};

pub(crate) enum Text {
    Plain(&'static str),
    Plural {
        input: &'static str,
        variants: &'static [(&'static str, &'static str)],
    },
}

include!(concat!(env!("OUT_DIR"), "/texts.rs"));

pub const AUTO: &str = "AUTO";
pub const FALLBACK: &str = "en";
const OTHER: &str = "*";

pub fn resolve(setting: &str) -> &'static str {
    if setting != AUTO {
        if let Some(tag) = known(setting) {
            return tag;
        }
    }

    sys_locale::get_locales()
        .find_map(|candidate| matched(&candidate))
        .unwrap_or(FALLBACK)
}

pub fn is_known(tag: &str) -> bool {
    known(tag).is_some()
}

fn known(tag: &str) -> Option<&'static str> {
    TEXTS
        .iter()
        .map(|(known, _)| *known)
        .find(|known| known.eq_ignore_ascii_case(tag))
}

fn matched(candidate: &str) -> Option<&'static str> {
    let lower = candidate
        .split(['.', '@'])
        .next()?
        .replace('_', "-")
        .to_ascii_lowercase();

    if lower.is_empty() || lower == "c" || lower == "posix" {
        return None;
    }

    if let Some(tag) = known(&lower) {
        return Some(tag);
    }

    if lower.starts_with("es") {
        return known(if lower.contains("es-es") {
            "es-ES"
        } else {
            "es-419"
        });
    }

    if lower.starts_with("pt") {
        return known(if lower.contains("br") {
            "pt-BR"
        } else {
            "pt-PT"
        });
    }

    if lower.starts_with("zh") {
        let traditional = ["tw", "hant", "hk"].iter().any(|part| lower.contains(part));

        return known(if traditional { "zh-TW" } else { "zh-CN" });
    }

    if lower.starts_with("no") || lower.starts_with("nn") {
        return known("nb");
    }

    known(lower.split('-').next()?)
}

fn lookup(language: &str, key: &str) -> Option<&'static Text> {
    TEXTS
        .iter()
        .find(|(tag, _)| *tag == language)?
        .1
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, text)| text)
}

// ! The categories of `Intl.PluralRules` for a whole number, the ones the
// dashboard picks its variant with.
fn plural_category(language: &str, count: i64) -> &'static str {
    let n = count.unsigned_abs();
    let (ten, hundred) = (n % 10, n % 100);
    let few = (2..=4).contains(&ten) && !(12..=14).contains(&hundred);

    match language {
        "zh-CN" | "zh-TW" | "ja" | "ko" | "th" | "vi" | "id" => "other",
        "ru" | "uk" if ten == 1 && hundred != 11 => "one",
        "ru" | "uk" | "pl" if few => "few",
        "ru" | "uk" => "many",
        "pl" if n == 1 => "one",
        "pl" => "many",
        "cs" if n == 1 => "one",
        "cs" if (2..=4).contains(&n) => "few",
        "ro" if n == 1 => "one",
        "ro" if n == 0 || (1..=19).contains(&hundred) => "few",
        "fr" | "pt-BR" if n <= 1 => "one",
        "fr" | "pt-BR" | "pt-PT" | "es-ES" | "es-419" | "it"
            if n != 0 && n.is_multiple_of(1_000_000) =>
        {
            "many"
        }
        _ if n == 1 => "one",
        _ => "other",
    }
}

// ! Mirrors the dashboard's own relative-time rule (minutes under an hour,
// then hours, then days), from the same `time_in_*` keys.
fn relative_in(language: &str, until: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let seconds = (until - now).num_seconds().max(0);
    let minutes = (seconds as f64 / 60.0).round() as i64;

    if minutes < 60 {
        return text(
            language,
            "time_in_minutes",
            &[("count", &minutes.to_string())],
        );
    }

    let hours = (seconds as f64 / 3600.0).round() as i64;

    if hours < 48 {
        return text(language, "time_in_hours", &[("count", &hours.to_string())]);
    }

    let days = (seconds as f64 / 86400.0).round() as i64;

    text(language, "time_in_days", &[("count", &days.to_string())])
}

fn pattern(language: &str, text: &'static Text, args: &[(&str, &str)]) -> &'static str {
    match text {
        Text::Plain(text) => text,
        Text::Plural { input, variants } => {
            let category = args
                .iter()
                .find(|(name, _)| name == input)
                .and_then(|(_, value)| value.parse::<i64>().ok())
                .map_or(OTHER, |count| plural_category(language, count));
            let variant = |wanted: &str| {
                variants
                    .iter()
                    .find(|(category, _)| *category == wanted)
                    .map(|(_, text)| *text)
            };

            variant(category)
                .or_else(|| variant(OTHER))
                .unwrap_or_default()
        }
    }
}

// ! One pass: a value is never read as a placeholder.
pub fn text(language: &str, key: &str, args: &[(&str, &str)]) -> String {
    let template = lookup(language, key)
        .map(|text| pattern(language, text, args))
        .or_else(|| lookup(FALLBACK, key).map(|text| pattern(FALLBACK, text, args)))
        .unwrap_or(key);
    let mut out = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let value = after.find('}').and_then(|close| {
            let name = &after[..close];
            args.iter()
                .find(|(arg, _)| *arg == name)
                .map(|(_, value)| (*value, close))
        });

        match value {
            Some((value, close)) => {
                out.push_str(value);
                rest = &after[close + 1..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }

    out.push_str(rest);

    out
}

pub fn number(language: &str, value: i64) -> String {
    let separator = match language {
        "fr" => "\u{202f}",
        "pt-PT" | "cs" | "pl" | "ru" | "uk" | "bg" | "hu" | "sv" | "nb" | "fi" => "\u{a0}",
        "de" | "nl" | "it" | "es-ES" | "pt-BR" | "id" | "tr" | "el" | "ro" | "da" | "vi" => ".",
        _ => ",",
    };

    // ! "1500", not "1 500": these languages group from five digits.
    let min_grouped = match language {
        "es-ES" | "pt-PT" | "it" | "pl" | "hu" | "bg" => 5,
        _ => 4,
    };

    let minus = match language {
        "sv" | "nb" | "fi" => "\u{2212}",
        _ => "-",
    };

    let digits = value.unsigned_abs().to_string();
    let grouped = digits.len() >= min_grouped;
    let mut out = String::new();

    if value < 0 {
        out.push_str(minus);
    }

    for (index, digit) in digits.chars().enumerate() {
        if grouped && index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push_str(separator);
        }

        out.push(digit);
    }

    out
}

pub fn kind_label(language: &str, kind: EventKind) -> String {
    let key = match kind {
        EventKind::Info => "event_kind_info",
        EventKind::Error => "event_kind_error",
        EventKind::WatchStarted => "event_kind_watch_started",
        EventKind::WatchStopped => "event_kind_watch_stopped",
        EventKind::DropProgress => "event_kind_drop_progress",
        EventKind::DropClaimed => "event_kind_drop_claimed",
        EventKind::CampaignFinished => "event_kind_campaign_finished",
        EventKind::CampaignSetAside => "event_kind_campaign_set_aside",
        EventKind::CampaignRetried => "event_kind_campaign_retried",
        EventKind::PointsEarned => "event_kind_points_earned",
        EventKind::PointsSpent => "event_kind_points_spent",
        EventKind::BonusClaimed => "event_kind_bonus_claimed",
        EventKind::PredictionPlaced => "event_kind_prediction_placed",
        EventKind::PredictionResult => "event_kind_prediction_result",
        EventKind::StreamOnline => "event_kind_stream_online",
        EventKind::StreamOffline => "event_kind_stream_offline",
        EventKind::RaidJoined => "event_kind_raid_joined",
        EventKind::MomentClaimed => "event_kind_moment_claimed",
        EventKind::StreakRecovered => "event_kind_streak_recovered",
        EventKind::GoalContributed => "event_kind_goal_contributed",
        EventKind::ChatMention => "event_kind_chat_mention",
        EventKind::SessionExpired => "event_kind_session_expired",
        EventKind::LoginCompleted => "event_kind_login_completed",
        EventKind::UpdateAvailable => "event_kind_update_available",
        EventKind::VersionUpdated => "event_kind_version_updated",
        EventKind::RewardRedeemed => "event_kind_reward_redeemed",
    };

    text(language, key, &[])
}

fn watch_points_key(reason: PointsReason) -> &'static str {
    match reason {
        PointsReason::StreakBonus => "ev_watch_points_streak_bonus",
        PointsReason::StreakAtRisk => "ev_watch_points_streak_at_risk",
        PointsReason::Subscribed => "ev_watch_points_subscribed",
        PointsReason::ListOrder => "ev_watch_points_list_order",
    }
}

pub fn event_text(language: &str, event: &MinerEvent) -> String {
    let context = &event.context;
    let channel = context
        .channel_name
        .as_deref()
        .or(context.channel.as_deref())
        .filter(|name| !name.is_empty());
    let drop = context.drop_name.as_deref();
    let campaign = context.campaign_name.as_deref();
    let count = |value: i64| number(language, value);
    let say = |key: &str, args: &[(&str, &str)]| Some(text(language, key, args));

    let sentence = match event.kind {
        EventKind::WatchStarted => match (channel, campaign, context.watch_reason) {
            (Some(channel), Some(campaign), _) => say(
                "ev_watch_drop",
                &[("channel", channel), ("campaign", campaign)],
            ),
            (Some(channel), None, Some(reason)) => {
                say(watch_points_key(reason), &[("channel", channel)])
            }
            _ => None,
        },
        EventKind::WatchStopped => {
            channel.and_then(|channel| say("ev_watch_stopped", &[("channel", channel)]))
        }
        EventKind::DropClaimed => match (drop, campaign) {
            (Some(drop), Some(campaign)) => {
                say("ev_drop_claimed", &[("drop", drop), ("campaign", campaign)])
            }
            _ => None,
        },
        EventKind::CampaignFinished => {
            campaign.and_then(|campaign| say("ev_campaign_finished", &[("campaign", campaign)]))
        }
        EventKind::DropProgress => match (drop, context.current, context.required) {
            (Some(drop), Some(current), Some(required)) => say(
                "ev_drop_progress",
                &[
                    ("drop", drop),
                    ("current", &count(i64::from(current))),
                    ("required", &count(i64::from(required))),
                ],
            ),
            (None, Some(current), Some(required)) => say(
                "ev_drop_progress_unknown",
                &[
                    ("current", &count(i64::from(current))),
                    ("required", &count(i64::from(required))),
                ],
            ),
            _ => None,
        },
        EventKind::BonusClaimed => {
            channel.and_then(|channel| say("ev_bonus", &[("channel", channel)]))
        }
        EventKind::PointsEarned => match (channel, context.earned) {
            (Some(channel), Some(earned)) => say(
                "ev_points",
                &[
                    ("channel", channel),
                    ("points", &count(earned)),
                    ("count", &earned.to_string()),
                ],
            ),
            _ => None,
        },
        EventKind::PointsSpent => match (channel, context.spent, context.balance) {
            (Some(channel), Some(spent), _) => say(
                "ev_points_spent",
                &[
                    ("channel", channel),
                    ("points", &count(spent)),
                    ("count", &spent.to_string()),
                ],
            ),
            (Some(channel), None, Some(balance)) => say(
                "ev_points_spent_balance",
                &[("channel", channel), ("balance", &count(balance))],
            ),
            _ => None,
        },
        EventKind::StreamOnline => {
            channel.and_then(|channel| say("ev_online", &[("channel", channel)]))
        }
        EventKind::StreamOffline => {
            channel.and_then(|channel| say("ev_offline", &[("channel", channel)]))
        }
        EventKind::RaidJoined => {
            channel.and_then(|channel| say("ev_raid", &[("channel", channel)]))
        }
        EventKind::MomentClaimed => {
            channel.and_then(|channel| say("ev_moment", &[("channel", channel)]))
        }
        EventKind::PredictionPlaced => match (channel, context.points) {
            (Some(channel), Some(points)) => say(
                "ev_bet_placed",
                &[
                    ("channel", channel),
                    ("points", &count(points)),
                    ("count", &points.to_string()),
                    ("outcome", context.outcome.as_deref().unwrap_or_default()),
                ],
            ),
            _ => None,
        },
        EventKind::PredictionResult => match context.bet_result {
            Some(BetResult::Win) => {
                let won = context.points_won.unwrap_or_default();

                say(
                    "ev_bet_won",
                    &[("points", &count(won)), ("count", &won.to_string())],
                )
            }
            Some(BetResult::Lose) => {
                let lost = context.staked.unwrap_or_default();

                say(
                    "ev_bet_lost",
                    &[("points", &count(lost)), ("count", &lost.to_string())],
                )
            }
            Some(BetResult::Refund) => say("ev_bet_refund", &[]),
            None => None,
        },
        EventKind::LoginCompleted => {
            let login = context
                .account_name
                .as_deref()
                .unwrap_or(event.account_login.as_str());

            say("ev_login", &[("login", login)])
        }
        EventKind::SessionExpired => say("ev_session_expired", &[]),
        EventKind::UpdateAvailable => context
            .version
            .as_deref()
            .and_then(|version| say("ev_update_available", &[("version", version)])),
        EventKind::VersionUpdated => match (
            context.version.as_deref(),
            context.previous_version.as_deref(),
        ) {
            (Some(version), Some(previous)) => say(
                "ev_version_updated",
                &[("version", version), ("previous", previous)],
            ),
            _ => None,
        },
        EventKind::StreakRecovered => {
            channel.and_then(|channel| say("ev_streak_recovered", &[("channel", channel)]))
        }
        EventKind::GoalContributed => match (channel, context.points) {
            (Some(channel), Some(points)) => say(
                "ev_goal_contributed",
                &[
                    ("channel", channel),
                    ("points", &count(points)),
                    ("count", &points.to_string()),
                    ("goal", context.goal.as_deref().unwrap_or_default()),
                ],
            ),
            _ => None,
        },
        EventKind::ChatMention => match (channel, context.chatter.as_deref()) {
            (Some(channel), Some(chatter)) => say(
                "ev_chat_mention",
                &[("channel", channel), ("chatter", chatter)],
            ),
            _ => None,
        },
        EventKind::Error => match (drop, context.status.as_deref(), &context.drop_id) {
            (Some(drop), Some(status), Some(_)) => {
                say("ev_claim_refused", &[("drop", drop), ("status", status)])
            }
            _ => None,
        },
        EventKind::CampaignSetAside => {
            match (campaign, context.required, context.set_aside_until) {
                (Some(campaign), Some(minutes), Some(until)) => say(
                    "ev_campaign_set_aside",
                    &[
                        ("campaign", campaign),
                        ("minutes", &count(i64::from(minutes))),
                        ("time", &relative_in(language, until, event.at)),
                    ],
                ),
                _ => None,
            }
        }
        EventKind::CampaignRetried => {
            campaign.and_then(|campaign| say("ev_campaign_retried", &[("campaign", campaign)]))
        }
        EventKind::Info | EventKind::RewardRedeemed => None,
    };

    sentence.unwrap_or_else(|| event.title.clone())
}
