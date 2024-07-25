use chrono::DateTime;
use chrono::Local;
use reqwest::Client;
use std::collections::HashMap;
use std::{thread, time};
use superconsole::components::Component;
use superconsole::components::DrawMode;
use superconsole::style::style;
use superconsole::style::Color;
use superconsole::style::Stylize;
use superconsole::Dimensions;
use superconsole::Line;
use superconsole::Lines;
use superconsole::SuperConsole;
use transit_rust::stop_monitor::{get_stops, MonitoredVehicleJourney};
use crossterm::{execute, terminal::{Clear, ClearType, EnterAlternateScreen}};
use std::io;


const RAPID_LINE_TO_PARENT_MAP: [(&str, &str); 2] = [("14R", "14"), ("8BX", "8")];

struct DisplayBoard {
    display_lines: Option<HashMap<String, Vec<MonitoredVehicleJourney>>>,
    last_successful_request_time: Option<DateTime<Local>>,
    last_request_successful: bool,
}

impl Component for DisplayBoard {
    fn draw_unchecked(&self, _dimensions: Dimensions, _mode: DrawMode) -> anyhow::Result<Lines> {
        let mut lines = Vec::new();

        if let Some(request_time) = self.last_successful_request_time {
            lines.push(Line::from_iter(vec![
                style("Last Updated: ".to_owned()).try_into()?,
                style(request_time.format("%Y-%m-%d %H:%M:%S").to_string())
                    .bold()
                    .try_into()?,
            ]));
        } else {
            lines.push(Line::from_iter(vec![style("No Information".to_owned()).with(Color::Red).try_into()?]))
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
        

        for key in sorted_keys {
            let mut this_vec = Vec::new();
            if key != "43" {
                this_vec.push(style(key.clone()).bold().try_into()?);
            } else {
                this_vec.push(style(key.clone()).bold().with(Color::Magenta).try_into()?);
            }
            let mut line_end = ":".to_owned();
            for _ in 0..(4 - key.chars().count()) {
                line_end.push_str(" ");
            }
            this_vec.push(style(line_end).bold().try_into()?);
            for mvj in &display_lines[&key] {
                match mvj.time_to_arrival() {
                    Some(tta) => {
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

#[tokio::main]
async fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut console = SuperConsole::new().unwrap();
    let client = Client::new();
    let mut rapid_to_line_map: HashMap<String, String> = HashMap::new();
    for (rapid, parent) in RAPID_LINE_TO_PARENT_MAP {
        rapid_to_line_map.insert(rapid.to_owned(), parent.to_owned());
    }

    let mut display_board = DisplayBoard {
        display_lines: None,
        last_successful_request_time: None,
        last_request_successful: false,
    };
    loop {
        let stops = match get_stops(&client).await {
            Ok(stops) => stops,
            Err(_) => {
                display_board.last_request_successful = false;
                execute!(io::stdout(), Clear(ClearType::All))?;
                console.render(&display_board).unwrap();
                thread::sleep(time::Duration::from_secs(10));
                continue;
            }
        };

        let mut display: HashMap<String, Vec<MonitoredVehicleJourney>> = HashMap::new();
        for (key, value) in stops.into_iter() {
            let parent_line = match rapid_to_line_map.get(&key.line_ref) {
                Some(parent_line) => (*parent_line).clone(),
                None => {
                    let line_ref = key.line_ref.clone();
                    line_ref
                }
            };
            for mvj in value {
                if let Some(_tta) = mvj.time_to_arrival() {
                    if let Some(x) = display.get_mut(&parent_line) {
                        x.push(mvj);
                    } else {
                        display.insert(parent_line.clone(), vec![mvj]);
                    }
                }
            }
        }
        let mut sorted_keys = Vec::new();

        let display_keys = display.keys().clone();
        for key in display_keys {
            sorted_keys.push(key.clone());
        }
        sorted_keys.sort();
        for key in sorted_keys {
            let value = display.get_mut(&key).unwrap();
            value.sort_by_key(|a| match a.time_to_arrival() {
                Some(v) => v,
                None => 999,
            });
        }
        display_board.display_lines = Some(display);
        display_board.last_successful_request_time = Some(Local::now());
        display_board.last_request_successful = true;
        execute!(io::stdout(), Clear(ClearType::All))?;
        console.render(&display_board).unwrap();
        
        thread::sleep(time::Duration::from_secs(30));
    }
}
