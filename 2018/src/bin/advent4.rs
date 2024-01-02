use advent2018::DataReader;
use text_io::*;

use fnv::FnvHashMap;
use prefix_sum::PrefixSum;

use std::fmt;

fn main() {
    let input = DataReader::open(4);

    let mut log = advent2018::time("Read input", || {
        let mut log = Vec::new();
        for line in input {
            log.push(GuardLogEntry::from(line));
        }
        log
    });
    advent2018::time("Sort input", || { log.sort_unstable(); });

    let log_map = advent2018::time("Log to map", || {
        log_into_days(&log)
    });
    let part_one = advent2018::time("Solve part one", || {
        let guard = find_most_minutes(&log_map);
        let minute = minute_most_asleep(&log_map[&guard]).0;
        guard.0 * minute as u32
    });
    let part_two = advent2018::time("Solve part two", || {
        let (a, b) = find_most_frequently_asleep(&log_map);
        a.0 * b
    });
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn find_most_frequently_asleep(log: &FnvHashMap<GuardId, Vec<GuardLogDay>>)
    -> (GuardId, u32)
{
    let mut res = None;
    for (&id, days) in log {
        let (minute, value) = minute_most_asleep(days);
        match res {
            None => res = Some((id, value, minute)),
            Some((_, ovalue, _)) => if ovalue < value {
                res = Some((id, value, minute));
            },
        }
    }
    let res = res.unwrap();
    (res.0, res.2 as u32)
}

fn find_most_minutes(log: &FnvHashMap<GuardId, Vec<GuardLogDay>>) -> GuardId {
    let mut best = None;
    for (&id, days) in log {
        let mut minutes_asleep = 0;
        for day in days {
            minutes_asleep += day.asleep_sum;
        }
        match best {
            None => best = Some((id, minutes_asleep)),
            Some((_id, omins)) => if minutes_asleep > omins {
                best = Some((id, minutes_asleep));
            }
        }
    }
    best.unwrap().0
}
fn minute_most_asleep(days: &[GuardLogDay]) -> (usize, usize) {
    let mut sum = PrefixSum::new(60);
    for day in days {
        for &(from, to) in &day.asleep {
            sum[from..to] += 1;
        }
    }
    let sum = sum.build();
    let mut max_min = std::usize::MAX;
    let mut max_sum = 0;
    for (min, sum) in sum.into_iter().enumerate() {
        if max_sum < sum {
            max_min = min;
            max_sum = sum;
        }
    }
    (max_min, max_sum)
}

struct GuardLogDay {
    asleep: Vec<(usize,usize)>,
    asleep_sum: usize,
}

fn log_into_days(entries: &[GuardLogEntry]) -> FnvHashMap<GuardId, Vec<GuardLogDay>> {
    let mut map = FnvHashMap::default();

    // Add vec for every GuardId.
    for entry in entries {
        if let GuardAction::BeginShift(id) = entry.action {
            // it's fine if we do this multiple times
            // remember: Vec::new doesn't allocate
            map.insert(id, Vec::new());
        }
    }

    enum State {
        NoGuard(),
        GuardAwake(GuardId, Vec<(usize,usize)>, usize),
        GuardAsleep(GuardId, Vec<(usize,usize)>, usize, usize),
    };

    let mut state = State::NoGuard();
    for entry in entries {
        state = match entry.action {
            GuardAction::BeginShift(new_id) => {
                match state {
                    State::NoGuard() => State::GuardAwake(new_id, Vec::new(), 0),
                    State::GuardAwake(prev_id, asleep, asleep_sum) => {
                        map.get_mut(&prev_id).unwrap().push(GuardLogDay {
                            asleep_sum,
                            asleep,
                        });
                        State::GuardAwake(new_id, Vec::new(), 0)
                    },
                    State::GuardAsleep(_, _, _, _)
                        => panic!("Guard asleep at end of shift."),
                }
            },
            GuardAction::FallAsleep => {
                match state {
                    State::NoGuard() => panic!("No guard to fall asleep!"),
                    State::GuardAwake(id, asleep, asleep_sum) => {
                        State::GuardAsleep(id, asleep, asleep_sum, entry.minute)
                    },
                    State::GuardAsleep(_, _, _, _) => panic!("Guard already asleep."),
                }
            },
            GuardAction::WakesUp => {
                match state {
                    State::NoGuard() => panic!("No guard to wake up!"),
                    State::GuardAwake(_, _, _) => panic!("Guard already awake."),
                    State::GuardAsleep(id, mut asleep, sum, sleep_minute) => {
                        asleep.push((sleep_minute, entry.minute));
                        State::GuardAwake(id, asleep, sum + (entry.minute-sleep_minute))
                    },
                }
            },
        };
    }
    match state {
        State::NoGuard() => {},
        State::GuardAwake(prev_id, asleep, asleep_sum) => {
            map.get_mut(&prev_id).unwrap().push(GuardLogDay {
                asleep_sum,
                asleep,
            });
        },
        State::GuardAsleep(_, _, _, _)
            => panic!("Guard asleep at end of time."),
    }
    map
}

#[derive(Eq,PartialEq,Ord,PartialOrd,Debug)]
struct GuardLogEntry {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    action: GuardAction,
}
impl GuardLogEntry {
    pub fn from(s: String) -> GuardLogEntry {
        let (year, month, day, hour, minute, action);
        let _ignore: String;
        scan!(s.bytes() => "[{}-{}-{} {}:{}] {} {}",
              year, month, day, hour, minute, _ignore, action);
        GuardLogEntry {
            year, month, day, hour, minute, action,
        }
    }
}
#[derive(Eq,PartialEq,Ord,PartialOrd,Debug)]
enum GuardAction {
    FallAsleep,
    WakesUp,
    BeginShift(GuardId),
}
impl std::str::FromStr for GuardAction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "asleep" => Ok(GuardAction::FallAsleep),
            "up" => Ok(GuardAction::WakesUp),
            _ => {
                let id;
                scan!(s.bytes() => "#{}", id);
                Ok(GuardAction::BeginShift(GuardId(id)))
            },
        }
    }
}
impl fmt::Display for GuardAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuardAction::FallAsleep => write!(f, "falls asleep"),
            GuardAction::WakesUp => write!(f, "wakes up"),
            GuardAction::BeginShift(i) => write!(f, "Guard #{} begins shift", i),
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash,PartialOrd,Ord)]
struct GuardId(u32);

impl fmt::Display for GuardId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


