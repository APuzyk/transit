use chrono::{DateTime, Local};
use std::collections::HashMap;

use superconsole::components::Component;
use superconsole::components::DrawMode;
use superconsole::style::style;
use superconsole::style::Color;
use superconsole::style::Stylize;
use superconsole::Dimensions;
use superconsole::Line;
use superconsole::Lines;
use superconsole::SuperConsole;

use crate::rgb_matrix::{RGBMatrix, RGBMatrixError, Font, FontError};
use crate::stop_monitor::MonitoredVehicleJourney;

pub struct DisplayBoard {
    pub display_lines: Option<HashMap<String, Vec<MonitoredVehicleJourney>>>,
    pub last_successful_request_time: Option<DateTime<Local>>,
    pub last_request_successful: bool,
    pub rgb_matrix: Option<RGBMatrix>,
    pub font: Option<Font>,
}

impl DisplayBoard {
    pub fn new() -> Self {
        DisplayBoard {
            display_lines: None,
            last_successful_request_time: None,
            last_request_successful: false,
            rgb_matrix: None,
            font: None,
        }
    }
    pub fn initialize_rgb_matrix(
        &mut self,
        rows: i32,
        chained: i32,
        parallel: i32,
    ) -> Result<(), RGBMatrixError> {
        match RGBMatrix::new(rows, chained, parallel) {
            Ok(matrix) => {
                self.rgb_matrix = Some(matrix);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    pub fn initialize_font<P: AsRef<Path>>(
        &mut self,
        font_file: P,
    ) -> Result<(), FontError> {
        match Font::new(font_file) {
            Ok(font) => {
                self.font = Some(font);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    pub fn test_write(self) {
        let mut canvas = self.rgb_matrix.get_canvas();
        canvas.draw_text(&self.font, 0, 10, 0, 0, 255, "Hello, World!", 0);
    }
}

impl Component for DisplayBoard {
    fn draw_unchecked(&self, _dimensions: Dimensions, _mode: DrawMode) -> anyhow::Result<Lines> {
        let mut lines = Vec::new();

        if let Some(request_time) = self.last_successful_request_time {
            lines.push(Line::from_iter(vec![
                style("Current Time: ".to_owned()).try_into()?,
                style(Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
                    .bold()
                    .try_into()?,
            ]));

            lines.push(Line::from_iter(vec![
                style("Last Updated: ".to_owned()).try_into()?,
                style(request_time.format("%Y-%m-%d %H:%M:%S").to_string())
                    .bold()
                    .try_into()?,
            ]));
        } else {
            lines.push(Line::from_iter(vec![style("No Information".to_owned())
                .with(Color::Red)
                .try_into()?]))
        }
        let display_lines = match &self.display_lines {
            Some(dl) => dl,
            None => return Ok(Lines(lines)),
        };

        let mut sorted_keys = Vec::new();
        for key in display_lines.keys() {
            sorted_keys.push(key.clone());
        }
        sorted_keys.sort();

        let longest_key = match sorted_keys.iter().map(|key| key.chars().count()).max() {
            Some(val) => val + 1,
            None => 0,
        };

        for key in sorted_keys {
            let mut this_vec = Vec::new();
            if key != "43" {
                this_vec.push(style(key.clone()).bold().try_into()?);
            } else {
                this_vec.push(style(key.clone()).bold().with(Color::Magenta).try_into()?);
            }
            let mut line_end = ":".to_owned();
            for _ in 0..(longest_key - key.chars().count()) {
                line_end.push_str(" ");
            }
            this_vec.push(style(line_end).bold().try_into()?);
            for mvj in &display_lines[&key] {
                match mvj.time_to_arrival() {
                    Some(tta) => {
                        // rapid lines the line ref isn't the same
                        // as the key
                        if mvj.line_ref != key {
                            if mvj.has_location() {
                                this_vec.push(
                                    style(tta.to_string()).with(Color::Red).bold().try_into()?,
                                );
                            } else {
                                this_vec.push(
                                    style(tta.to_string())
                                        .with(Color::Red)
                                        .italic()
                                        .try_into()?,
                                );
                            }
                        } else {
                            if mvj.has_location() {
                                this_vec.push(
                                    style(tta.to_string()).with(Color::Blue).bold().try_into()?,
                                );
                            } else {
                                this_vec.push(
                                    style(tta.to_string())
                                        .with(Color::Blue)
                                        .italic()
                                        .try_into()?,
                                );
                            }
                        }
                    }
                    None => (),
                }
                this_vec.push(style(" ".to_owned()).try_into()?);
            }
            lines.push(Line::from_iter(this_vec));
        }
        Ok(Lines(lines))
    }
}
