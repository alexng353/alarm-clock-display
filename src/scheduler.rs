use rp2040_hal::{fugit::MicrosDurationU64, timer::Instant};

#[derive(Clone, Copy)]
pub enum Task {
    Refresh,
    ChangeTime,
    GetTime,
}

#[derive(Clone, Copy)]
pub struct Schedule {
    pub task: Task,
    deadline_us: Instant,
    interval_us: MicrosDurationU64,
}

pub struct Scheduler {
    schedules: [Schedule; 3],
}

impl Scheduler {
    pub fn new(now: Instant) -> Self {
        Self {
            schedules: [
                Schedule {
                    task: Task::Refresh,
                    deadline_us: now,
                    interval_us: MicrosDurationU64::millis(1),
                },
                Schedule {
                    task: Task::ChangeTime,
                    deadline_us: now,
                    interval_us: MicrosDurationU64::millis(100),
                },
                Schedule {
                    task: Task::GetTime,
                    deadline_us: now,
                    interval_us: MicrosDurationU64::secs(1),
                },
            ],
        }
    }

    pub fn next(&mut self, now: Instant) -> Option<Schedule> {
        match self
            .schedules
            .iter_mut()
            .enumerate()
            .min_by_key(|(_, x)| x.deadline_us)
        {
            Some((_, schedule)) => {
                if schedule.deadline_us <= now {
                    schedule.deadline_us += schedule.interval_us;
                    Some(*schedule)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
