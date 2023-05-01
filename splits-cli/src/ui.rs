use std::time::Duration;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};
use splits_core::Split;

use crate::{app::App, style};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(9), Constraint::Length(1)])
        .split(frame.size());

    let items: Vec<Row> = app
        .run
        .segments()
        .iter()
        .zip(
            app.timer
                .splits()
                .iter()
                .map(Some)
                .chain(std::iter::repeat(None)),
        )
        .enumerate()
        .map(|(index, (segment, split))| {
            let title = Span::raw(segment.title());
            let mut row = vec![title];
            let time = match split {
                Some(split) => match split {
                    Split::Skipped => None,
                    Split::Split(time) => Some(*time),
                },
                None => {
                    if app.timer.current_index() == Some(index) {
                        Some(app.timer.current_time())
                    } else {
                        segment.best_time()
                    }
                }
            };
            if let Some(time) = time {
                let style = match segment.best_time() {
                    Some(best) => match time.cmp(&best) {
                        std::cmp::Ordering::Less => style::green(),
                        std::cmp::Ordering::Equal => style::yellow(),
                        std::cmp::Ordering::Greater => style::red(),
                    },
                    None => style::white(),
                };

                if let Some(best) = segment.best_time() {
                    if best != time {
                        let diff = time.as_secs_f64() - best.as_secs_f64();
                        row.push(Span::styled(format!("{:+.2}", diff), style));
                    } else {
                        row.push(Span::raw(""));
                    }
                } else {
                    row.push(Span::raw(""));
                }
                row.push(Span::raw(format_time(time)));
            }
            Row::new(row)
        })
        .collect();

    let table = Table::new(items)
        .block(Block::default().borders(Borders::ALL))
        .widths(&[
            Constraint::Length(16),
            Constraint::Length(5),
            Constraint::Length(5),
        ]);

    frame.render_widget(table, chunks[0]);
    frame.render_widget(
        Paragraph::new(Span::raw(format_time(app.timer.current_time()))),
        chunks[1],
    );
}

fn format_time(duration: Duration) -> String {
    format!("{:.2}", duration.as_secs_f64())
}
