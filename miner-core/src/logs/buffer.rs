use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::{LogLevel, LogLine};

pub const CAPACITY: usize = 4000;

#[derive(Default)]
struct Ring {
    lines: VecDeque<LogLine>,
    next_id: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LogFilter<'a> {
    pub accounts: Option<&'a [String]>,
    pub levels: Option<&'a [LogLevel]>,
}

impl LogFilter<'_> {
    fn keeps(&self, line: &LogLine) -> bool {
        self.accounts.is_none_or(|accounts| {
            line.account
                .as_deref()
                .is_some_and(|account| accounts.iter().any(|a| a == account))
        }) && self
            .levels
            .is_none_or(|levels| levels.contains(&line.level))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogPage {
    pub lines: Vec<LogLine>,
    pub next_before_id: Option<u64>,
}

#[derive(Clone, Default)]
pub struct LogBuffer {
    ring: Arc<Mutex<Ring>>,
}

impl LogBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, line: LogLine) {
        let mut ring = crate::util::locked(&self.ring);
        let id = ring.next_id;
        ring.next_id += 1;

        if ring.lines.len() == CAPACITY {
            ring.lines.pop_front();
        }

        ring.lines.push_back(LogLine { id, ..line });
    }

    pub fn page(&self, filter: &LogFilter, before_id: Option<u64>, limit: usize) -> LogPage {
        let ring = crate::util::locked(&self.ring);
        let mut older = ring
            .lines
            .iter()
            .rev()
            .filter(|l| before_id.is_none_or(|before| l.id < before))
            .filter(|l| filter.keeps(l));
        let mut lines: Vec<LogLine> = older.by_ref().take(limit).cloned().collect();

        let next_before_id = match (lines.last(), older.next()) {
            (Some(oldest), Some(_)) => Some(oldest.id),
            _ => None,
        };

        lines.reverse();

        LogPage {
            lines,
            next_before_id,
        }
    }

    pub fn lines(&self, filter: &LogFilter, limit: usize) -> Vec<LogLine> {
        self.page(filter, None, limit).lines
    }

    pub fn export(&self, filter: &LogFilter, limit: usize, zone: chrono_tz::Tz) -> String {
        self.lines(filter, limit)
            .iter()
            .map(|l| {
                format!(
                    "{} {:<5} {}{}",
                    crate::util::format_local(l.at, zone),
                    l.level,
                    l.account
                        .as_ref()
                        .map(|a| format!("[{a}] "))
                        .unwrap_or_default(),
                    l.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
