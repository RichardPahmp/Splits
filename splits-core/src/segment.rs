use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Segment {
    pub(crate) title: String,
    pub(crate) history: Vec<Duration>,
    pub(crate) best_time: Option<Duration>,
    pub(crate) best_segment: Option<Duration>,
}

impl Segment {
    pub fn new(name: &str) -> Self {
        Self {
            title: name.into(),
            history: Vec::new(),
            best_time: None,
            best_segment: None,
        }
    }

    pub fn load(
        title: String,
        history: Vec<Duration>,
        best_time: Option<Duration>,
        best_segment: Option<Duration>,
    ) -> Self {
        Self {
            title,
            history,
            best_time,
            best_segment,
        }
    }

    pub fn history(&self) -> &[Duration] {
        &self.history
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn best_time(&self) -> Option<Duration> {
        self.best_time
    }

    pub fn best_segment(&self) -> Option<Duration> {
        self.best_segment
    }

    pub fn add_time(&mut self, time: Duration) {
        match self.best_time {
            Some(best) => {
                if time < best {
                    self.best_time = Some(time);
                }
            }
            None => {
                self.best_time = Some(time);
            }
        }
        self.history.push(time);
    }

    pub fn add_segment_time(&mut self, time: Duration) {
        match self.best_segment {
            Some(best) => {
                if time < best {
                    self.best_segment = Some(time);
                }
            }
            None => {
                self.best_segment = Some(time);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Segment;

    #[test]
    fn add_time() {
        let mut segment = Segment::new("test");
        assert_eq!(segment.best_time(), None);
        segment.add_time(Duration::from_secs(3));
        assert_eq!(segment.best_time(), Some(Duration::from_secs(3)));
        segment.add_time(Duration::from_secs(5));
        assert_eq!(segment.best_time(), Some(Duration::from_secs(3)));
        segment.add_time(Duration::from_secs(2));
        assert_eq!(segment.best_time(), Some(Duration::from_secs(2)));
    }
}
