use std::time::Duration;

use crate::{segment::Segment, timer::Split};

pub struct Run {
    title: String,
    segments: Vec<Segment>,
}

impl Run {
    pub fn new(title: String, segments: Vec<Segment>) -> Self {
        Self { title, segments }
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    pub fn update(&mut self, splits: &[Split]) {
        self.segments
            .iter_mut()
            .zip(splits.iter())
            .for_each(|(segment, split)| {
                if let Split::Split(time) = split {
                    segment.add_time(*time);
                }
            });
        self.update_segments(splits);
    }

    fn update_segments(&mut self, splits: &[Split]) {
        let mut last_time = Some(Duration::ZERO);

        for (segment, split) in self.segments.iter_mut().zip(splits) {
            if let Split::Split(time) = split {
                if let Some(last) = last_time {
                    if last < *time {
                        segment.add_segment_time(*time - last);
                    }
                }
                last_time = Some(*time);
            } else {
                last_time = None;
            }
        }
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn best_time(&self) -> Option<Duration> {
        self.segments.last().and_then(|segment| segment.best_time)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.segments.len()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::timer::Split;
    use crate::{Run, Segment};

    fn run() -> Run {
        Run::new(
            "test".to_string(),
            vec![Segment::new("a"), Segment::new("b"), Segment::new("c")],
        )
    }

    macro_rules! duration {
        ( $secs:literal s ) => {
            Duration::from_secs_f32($secs as f32)
        };

        ( $ms:literal ms ) => {
            Duration::from_millis($ms)
        };
    }

    macro_rules! splits {
        (@array [$($prev:expr,)*]) => {
            &[$($prev,)*]
        };

        (@array [$($prev:expr),*]) => {
            &[$($prev,)*]
        };

        (@array [$($prev:expr),*] skip $($rest:tt)*) => {
            splits!(@array [$($prev,)* Split::Skipped] $($rest)*)
        };

        (@array [$($prev:expr),*] $l:literal s $($rest:tt)*) => {
            splits!(@array [$($prev,)* splits!($l s)] $($rest)*)
        };

        (@array [$($prev:expr),*] $l:literal ms $($rest:tt)*) => {
            splits!(@array [$($prev,)* splits!($l ms)] $($rest)*)
        };

        (@array [$($prev:expr),*] , $($rest:tt)*) => {
            splits!(@array [$($prev),*] $($rest)*)
        };

        ($l:literal $s:ident) => {
            Split::Split(duration!($l $s))
        };

        ( $($tt:tt)+ ) => {
            splits!(@array [] $($tt)+)
        };
    }

    #[test]
    fn history() {
        let mut run = run();
        assert_eq!(run.segments[0].best_time, None);
        assert_eq!(run.segments[1].best_time, None);
        assert_eq!(run.segments[2].best_time, None);
        run.update(splits![
            3 s,
            5 s,
            7 s
        ]);
        assert_eq!(run.segments[0].best_time, Some(duration!(3 s)));
        assert_eq!(run.segments[1].best_time, Some(duration!(5 s)));
        assert_eq!(run.segments[2].best_time, Some(duration!(7 s)));
        run.update(splits![
            2 s,
            7 s,
            4 s
        ]);
        assert_eq!(run.segments[0].best_time, Some(duration!(2 s)));
        assert_eq!(run.segments[1].best_time, Some(duration!(5 s)));
        assert_eq!(run.segments[2].best_time, Some(duration!(4 s)));
    }

    #[test]
    fn best_segment() {
        let mut run = run();
        assert_eq!(run.segments[0].best_segment, None);
        assert_eq!(run.segments[1].best_segment, None);
        assert_eq!(run.segments[2].best_segment, None);
        run.update(splits![
            1 s,
            3 s,
            6 s,
        ]);
        assert_eq!(run.segments[0].best_segment, Some(duration!(1 s)));
        assert_eq!(run.segments[1].best_segment, Some(duration!(2 s)));
        assert_eq!(run.segments[2].best_segment, Some(duration!(3 s)));
        run.update(splits![
            3 s,
            4 s,
            10 s,
        ]);
        assert_eq!(run.segments[0].best_segment, Some(duration!(1 s)));
        assert_eq!(run.segments[1].best_segment, Some(duration!(1 s)));
        assert_eq!(run.segments[2].best_segment, Some(duration!(3 s)));
    }

    #[test]
    fn skip_does_not_record_best_segment() {
        let mut run = run();
        run.update(splits![
            2 s,
            5 s,
            8 s,
        ]);
        run.update(splits![
            1 s,
            skip,
            3 s,
        ]);
        assert_eq!(run.segments[0].best_segment, Some(duration!(1 s)));
        assert_eq!(run.segments[1].best_segment, Some(duration!(3 s)));
        assert_eq!(run.segments[2].best_segment, Some(duration!(3 s)));
    }
}
