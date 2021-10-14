use aoc::*;
use chrono::{DateTime, TimeZone, Timelike, Utc};

use std::{cmp::Ordering, collections::HashMap, str::FromStr};

type Id = usize;
type Schedule = [usize; 60];

#[derive(Debug)]
pub struct Guard {
    pub id: Id,
    pub sleeping_schedule: Schedule,
}

impl Guard {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            sleeping_schedule: [0; 60],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub timestamp: DateTime<Utc>,
    pub state: State,
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum State {
    Sleeping,
    Awake,
    Begin(Id),
}

impl FromStr for State {
    type Err = Error;
    fn from_str(log: &str) -> Result<Self, Self::Err> {
        match log {
            "wakes up" => Ok(Self::Awake),
            "falls asleep" => Ok(Self::Sleeping),
            log => Ok(Self::Begin(
                log.strip_prefix("Guard #")
                    .unwrap()
                    .strip_suffix(" begins shift")
                    .unwrap()
                    .parse()?,
            )),
        }
    }
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("] ").collect();

        Ok(Self {
            timestamp: Utc.datetime_from_str(parts[0], "[%Y-%m-%d %H:%M")?,
            state: parts[1].parse()?,
        })
    }
}

pub fn parse() -> HashMap<Id, Schedule> {
    let mut records: Vec<_> = parser::lines::<Record>().collect();
    records.sort();

    let start = match records[0] {
        Record {
            state: State::Begin(id),
            timestamp,
        } => (Guard::new(id), timestamp),
        _ => panic!(),
    };

    records
        .iter()
        .scan(start, |(guard, timestamp), record| match record.state {
            State::Sleeping => {
                *timestamp = record.timestamp;
                Some(None)
            }
            State::Awake => {
                (timestamp.minute()..record.timestamp.minute())
                    .for_each(|minute| guard.sleeping_schedule[minute as usize] += 1);
                *timestamp = record.timestamp;
                Some(None)
            }
            State::Begin(id) => Some(Some(std::mem::replace(guard, Guard::new(id)))),
        })
        .filter_map(|guard| guard)
        .fold(HashMap::new(), |mut map, guard| {
            map.entry(guard.id)
                .or_insert([0; 60])
                .iter_mut()
                .zip(guard.sleeping_schedule)
                .for_each(|(result, guard)| *result += guard);
            map
        })
}
