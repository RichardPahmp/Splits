use std::collections::HashMap;


pub trait TimeProvider {
    type Instant: Copy + Clone;
    type Duration: Copy + Clone;

    fn now() -> Self::Instant;
    fn elapsed(since: Self::Instant) -> Self::Duration;
}

pub struct StdTime;

impl TimeProvider for StdTime {
    type Instant = std::time::Instant;
    type Duration = std::time::Duration;

    fn now() -> Self::Instant {
        Self::Instant::now()
    }

    fn elapsed(since: Self::Instant) -> Self::Duration {
        since.elapsed()
    }
}

#[cfg(test)]
pub mod mock {
    use std::cell::RefCell;

    use super::TimeProvider;

    thread_local!(static TIME: RefCell<i32> = RefCell::new(0));

    #[derive(Debug, Default)]
    pub struct MockTime;

    impl MockTime {
        pub fn step(time: i32) {
            TIME.with(|now| {
                *now.borrow_mut() += time;
            });
        }
    }

    impl TimeProvider for MockTime {
        type Instant = i32;
        type Duration = i32;

        fn now() -> Self::Instant {
            TIME.with(|now| *now.borrow())
        }

        fn elapsed(instant: Self::Instant) -> Self::Duration {
            TIME.with(|now| *now.borrow()) - instant
        }
    }

    #[test]
    fn mock_time() {
        assert_eq!(MockTime::now(), 0);
        MockTime::step(5);
        assert_eq!(MockTime::now(), 5);
        let t = MockTime::now();
        MockTime::step(10);
        assert_eq!(MockTime::elapsed(t), 10);
    }
}