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

pub struct RGBDisplayLine {
    line: Vec<StyledString>,
}

impl RGBDisplayLine {
    pub fn new() -> Self {
        RBDDisplayLine {
            line: Vec<StyledString>::new(),
        }
    }
}

pub struct StyledString {
    string: String,
    color: Color,
}

impl StyledString {
    pub fn new() -> Self {
        StyledString {
            string: "".to_string(),
            color: LedColor::new(),
        }
    }
}

pub struct LedColor {
    red: i32,
    green: i32,
    blue: i32,
}

impl LedColor {
    pub fn new() -> Self {
        LedColor {
            red: 0, 
            green: 0, 
            blue: 0,
        }
    }

    pub fn red() -> Self {
        LedColor {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub fn green() -> Self {
        LedColor{
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub fn blue() -> Self {
        LedColor {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

const COL_WIDTH: i32 = 64;

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
        canvas.draw_text(&self.font, 0, 6, 0, 0, 255, "Hello, World!", 0);
    }
    pub fn write_lines(self) {
        self.write_times();
        self.write_bus_times();
    }
    pub fn write_times(self) {
        let mut canvas = self.rgb_matrix.get_canvas();

        let font_height = self.font.height();
        curr_row = font_height;
        curr_time = String::from("Current Time: ");
        let now = Local::now();
        if (now.month() == 2 && now.day() == 2) {
            curr_time.push_str(String::from("YOUR BIRTHDAY!"));
        } else {
            curr_time.push_str(now.format("%H:%M:%S").to_string());
        }
        canvas.draw_text(&self.font, 0, curr_row, 0, 0, 255, curr_time, 0);

        if let Some(request_time) = self.last_successful_request_time {
            last_updated = String::from("Last Updated: ");
            last_updated.push_str(request_time.format("%H:%M:%S").to_string());
            canvas.draw_text(&self.font, COL_WIDTH, curr_row, 0, 0, 255, last_updated, 0);
        }


        curr_row += font_height + 1;

        let lines_to_write = self.get_bus_styled_lines();
        
        for (index, line) in lines_to_write.iter().enumerate() {
            for styled_str in line.line {
                let col_pos = if index % 2 == 1 {0} else {COL_WIDTH};
                canvas.draw_text(&self.font, col_pos, cur_row, styled_str.color.red, styled_str.color.green, styled_str.color.blue, styled_str.string, 0);
                if index %2 == 0 {
                    curr_row += font_height + 1;
                }
            } 
        }
    }

    pub fn lines_to_write(self) -> Vec<String> {
        
    }
    pub fn get_bus_styled_lines(self) -> Vec<RGBDisplayLine> {
        let lines = Vec<RGBDisplayLine>::new();
        let display_lines = match &self.display_lines {
            Some(dl) => dl,
            None => return output,
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
            let mut this_line = RGBDisplayLine::new();
            this_line.line.push(
                StyledString {
                    string: key.clone(),
                    color: LedColor.blue(),
                },
            );

            this_line.line.push(
                StyledString{
                    string: ":".to_string(),
                    color: LedColor.blue(),
                }
            );
            
            for _ in 0..(longest_key - key.chars().count()) {
                this_line.line.push(
                    StyledString{
                        string: " ".to_string(),
                        color: LedColor.blue(),
                    }
                );
            }
            for mvj in &display_lines[&key] {
                match mvj.time_to_arrival() {
                    Some(tta) => {
                        // rapid lines the line ref isn't the same
                        // as the key
                        if mvj.line_ref != key {
                            if mvj.has_location() {
                                this_line.line.push(
                                    StyledString {
                                        string: tta.to_string(),
                                        color: LedColor.red(),
                                    }
                                );
                            } else {
                                this_line.line.push(
                                    StyledString {
                                        string: tta.to_string(),
                                        color: LedColor.red()
                                    }
                                );
                            }
                        } else {
                            if mvj.has_location() {
                                this_line.line.push(
                                    StyledString {
                                        string: tta.to_string(),
                                        color: LedColor.blue(),
                                    }
                                );
                            } else {
                                this_line.line.push(
                                    StyledString {
                                        string: tta.to_string(),
                                        color: LedColor.blue(),
                                    }
                                );
                            }
                        }
                    }
                    None => (),
                }
                this_line.line.push(
                    StyleString {
                        string: " ".to_string(),
                        color: LedColor.blue(),
                    }
                )
            }
            lines.push(this_line);
        }
        return lines
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
