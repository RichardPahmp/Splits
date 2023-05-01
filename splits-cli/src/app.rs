use std::{
    path::{PathBuf},
};

use splits_core::{Run, Timer};

use crate::file::{load_run, save_run};

pub struct App {
    pub timer: Timer,
    pub run: Run,
    pub should_exit: bool,
    pub splits_file: PathBuf,
}

impl App {
    pub fn new<P: Into<PathBuf>>(run: Run, splits_file: P) -> Self {
        Self {
            timer: Timer::new(run.len()),
            run,
            should_exit: false,
            splits_file: splits_file.into(),
        }
    }

    pub fn from_file<P: Into<PathBuf>>(path: P) -> anyhow::Result<Self> {
        let path = path.into();
        let run = load_run(&path)?;
        Ok(Self::new(run, path))
    }

    pub fn on_key(&mut self, c: char) -> anyhow::Result<()> {
        match c {
            'q' => self.should_exit = true,
            ' ' => self.timer.start_split_or_unpause(),
            's' => self.timer.skip(),
            'p' => self.timer.toggle_pause(),
            'u' => self.timer.undo(),
            'r' => self.save_and_reset()?,
            _ => {}
        }
        Ok(())
    }

    fn save_and_reset(&mut self) -> anyhow::Result<()> {
        self.run.update(self.timer.splits());
        save_run(&self.splits_file, &self.run)?;
        self.timer.reset();
        Ok(())
    }
}
