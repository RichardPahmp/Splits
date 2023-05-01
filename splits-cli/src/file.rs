use std::{fs, path::Path};

use splits_core::Run;
use splits_serde::RunSchema;

pub fn load_run(file: &Path) -> anyhow::Result<Run> {
    Ok(Run::from(serde_yaml::from_str::<RunSchema>(
        &fs::read_to_string(file)?,
    )?))
}

pub fn save_run(file: &Path, run: &Run) -> anyhow::Result<()> {
    fs::write(file, serde_yaml::to_string(&RunSchema::from(run))?)?;
    Ok(())
}
