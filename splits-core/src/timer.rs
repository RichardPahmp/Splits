use std::time::{Duration, Instant};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimerState {
    NotStarted,
    Running,
    Paused(Instant),
    Finished,
}

#[derive(Copy, Clone, Debug)]
pub enum Split {
    Skipped,
    Split(Duration),
}

impl Split {
    pub fn unwrap_time(self) -> Duration {
        match self {
            Split::Skipped => panic!("unwrap_time on skipped split!"),
            Split::Split(time) => time,
        }
    }
}

pub struct Timer {
    start_time: Instant,
    paused_time: Duration,
    num_splits: usize,
    splits: Vec<Split>,
    state: TimerState,
}

impl Timer {
    pub fn new(num_splits: usize) -> Self {
        Self {
            num_splits,
            start_time: Instant::now(),
            paused_time: Duration::ZERO,
            splits: Vec::with_capacity(num_splits),
            state: TimerState::NotStarted,
        }
    }

    pub fn state(&self) -> TimerState {
        self.state
    }

    pub fn start(&mut self) {
        if self.state == TimerState::NotStarted {
            self.start_time = Instant::now();
            self.state = TimerState::Running;
        }
    }

    pub fn start_split_or_unpause(&mut self) {
        match self.state {
            TimerState::NotStarted => self.start(),
            TimerState::Running => self.split(),
            TimerState::Paused(_) => self.unpause(),
            _ => {}
        }
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.state = TimerState::Paused(Instant::now());
        }
    }

    pub fn unpause(&mut self) {
        if let TimerState::Paused(paused_at) = self.state {
            self.paused_time += paused_at.elapsed();
            self.state = TimerState::Running;
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            TimerState::Running => self.pause(),
            TimerState::Paused(_) => self.unpause(),
            _ => {}
        }
    }

    pub fn split(&mut self) {
        if self.state == TimerState::Running {
            self.splits.push(Split::Split(self.current_time()));
            if self.splits.len() == self.num_splits {
                self.state = TimerState::Finished;
            }
        }
    }

    pub fn skip(&mut self) {
        if let TimerState::Running | TimerState::Paused(_) = self.state {
            if self.splits.len() != self.num_splits - 1 {
                self.splits.push(Split::Skipped);
            }
        }
    }

    pub fn undo(&mut self) {
        if self.state != TimerState::NotStarted && !self.splits.is_empty() {
            if self.state == TimerState::Finished {
                self.state = TimerState::Running;
            }
            self.splits.pop();
        }
    }

    pub fn current_time(&self) -> Duration {
        match self.state {
            TimerState::NotStarted => Duration::ZERO,
            TimerState::Running => self.start_time.elapsed() - self.paused_time,
            TimerState::Paused(paused_at) => {
                paused_at.duration_since(self.start_time) - self.paused_time
            }
            TimerState::Finished => self.splits.last().unwrap().unwrap_time(),
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::NotStarted;
        self.splits.clear();
    }

    pub fn current_index(&self) -> Option<usize> {
        if self.state == TimerState::NotStarted {
            None
        } else {
            Some(self.splits.len())
        }
    }

    pub fn get_segment_times(&self) -> Vec<Option<Duration>> {
        self.splits()
            .iter()
            .tuple_windows()
            .map(|tuple| match tuple {
                (Split::Split(first), Split::Split(second)) => Some(*second - *first),
                _ => None,
            })
            .collect()
    }

    pub fn splits(&self) -> &[Split] {
        &self.splits
    }
}

#[cfg(test)]
mod tests {
    use crate::{Timer, TimerState};

    #[test]
    fn splits() {
        let mut timer = Timer::new(3);
        assert_eq!(timer.state(), TimerState::NotStarted);
        timer.start();
        assert_eq!(timer.state(), TimerState::Running);
        timer.split();
        assert_eq!(timer.state(), TimerState::Running);
        timer.split();
        assert_eq!(timer.state(), TimerState::Running);
        timer.split();
        assert_eq!(timer.state(), TimerState::Finished);
    }

    #[test]
    fn test() {
        let mut timer = Timer::new(5);
        timer.start();
        for _ in 0..100 {
            println!("{:?}", timer.current_time());
        }
    }
}
