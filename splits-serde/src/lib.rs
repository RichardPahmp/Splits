use std::time::Duration;

use serde::{Deserialize, Serialize};
use splits_core as core;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunSchema {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_time: Option<f64>,
    segments: Vec<SegmentSchema>,
}

impl From<&core::Run> for RunSchema {
    fn from(run: &core::Run) -> Self {
        Self {
            title: run.title().to_string(),
            best_time: run.best_time().map(|d| d.as_secs_f64()),
            segments: run.segments().iter().map(SegmentSchema::from).collect(),
        }
    }
}

impl From<RunSchema> for core::Run {
    fn from(run: RunSchema) -> Self {
        core::Run::new(
            run.title,
            run.segments.into_iter().map(From::from).collect(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentSchema {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_segment: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    history: Vec<f64>,
}

impl From<&core::Segment> for SegmentSchema {
    fn from(segment: &core::Segment) -> Self {
        Self {
            title: segment.title().to_string(),
            best_time: segment.best_time().map(|d| d.as_secs_f64()),
            best_segment: segment.best_segment().map(|d| d.as_secs_f64()),
            history: segment.history().iter().map(|d| d.as_secs_f64()).collect(),
        }
    }
}

impl From<SegmentSchema> for core::Segment {
    fn from(segment: SegmentSchema) -> Self {
        Self::load(
            segment.title,
            segment
                .history
                .iter()
                .copied()
                .map(Duration::from_secs_f64)
                .collect(),
            segment.best_time.map(Duration::from_secs_f64),
            segment.best_segment.map(Duration::from_secs_f64),
        )
    }
}
