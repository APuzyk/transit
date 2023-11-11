use reqwest::Client;
use std::collections::HashMap;
use std::fmt::Display;
use std::{thread, time};
use transit_rust::stop_monitor::{get_stops, MonitoredVehicleJourney};
use colored::Colorize;
use derive_more::Display;
use superconsole::components::Blank;
use superconsole::components::Component;
use superconsole::components::DrawMode;
use superconsole::style::style;
use superconsole::style::Color;
use superconsole::style::Stylize;
use superconsole::Dimensions;
use superconsole::Line;
use superconsole::Lines;
use superconsole::Span;
use superconsole::SuperConsole;
use chrono::Local;

const RAPID_LINE_TO_PARENT_MAP: [(&str, &str); 2] = [("14R", "14"), ("8BX", "8")];

struct DisplayBoard {
    display_lines: HashMap<String, Vec<MonitoredVehicleJourney>>
}

impl Component for DisplayBoard {
    fn draw_unchecked(&self, _dimensions: Dimensions, _mode: DrawMode) -> anyhow::Result<Lines> {
        let mut sorted_keys = Vec::new();
        for key in self.display_lines.keys() {
            sorted_keys.push(key.clone());
        }
        sorted_keys.sort();
        let mut lines = Vec::new();
        let date = Local::now();
        lines.push(Line::from_iter(vec![style(date.format("%Y-%m-%d %H:%M").to_string()).bold().try_into()?]));
        for key in sorted_keys {
            let mut this_vec = Vec::new();
            this_vec.push(style(key.clone()).bold().try_into()?);
            this_vec.push(style(" ".to_owned()).try_into()?);
            for mvj in &self.display_lines[&key] {
                match mvj.time_to_arrival() {
                    Some(tta) => {
                        if mvj.line_ref != key{
                            this_vec.push(style(tta.to_string()).with(Color::Red).try_into()?);
                            if mvj.has_location() {
                                this_vec.push(style("*".to_owned()).with(Color::Red).try_into()?);
                            }
                        } else {
                            this_vec.push(style(tta.to_string()).with(Color::Blue).try_into()?);
                            if mvj.has_location() {
                                this_vec.push(style("*".to_owned()).with(Color::Blue).try_into()?);
                            }
                        }
                    },
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
async fn main() {
    let mut console = SuperConsole::new().unwrap();
    let client = Client::new();
    let mut rapid_to_line_map: HashMap<String, String> = HashMap::new();
    for (rapid, parent) in RAPID_LINE_TO_PARENT_MAP {
        rapid_to_line_map.insert(rapid.to_owned(), parent.to_owned());
    }

    loop {
        let mut display: HashMap<String, Vec<MonitoredVehicleJourney>> = HashMap::new();
        let stops = get_stops(&client).await;
        for (key, value) in stops.into_iter() {
            // print!("{}: ", key.screen_display());
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
            // println!("");
        }
        let mut sorted_keys = Vec::new();
        for key in display.keys() {
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
        let display_board = DisplayBoard {
            display_lines: display
        };
        console
            .render(&display_board)
            .unwrap();



        thread::sleep(time::Duration::from_secs(30));
    }
}
