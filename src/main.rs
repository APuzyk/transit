use chrono::Local;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen},
};
use reqwest::Client;
use std::collections::HashMap;
use std::io;
use std::{thread, time};
use superconsole::SuperConsole;
use transit_rust::constants::RAPID_LINE_TO_PARENT_LINE_MAP;
use transit_rust::display_board::DisplayBoard;
use transit_rust::stop_monitor::{get_stops, LineStop, MonitoredVehicleJourney};

#[tokio::main]
async fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut console = SuperConsole::new().unwrap();
    let client = Client::new();
    let rapid_line_to_parent_map: HashMap<&str, &str> =
        HashMap::from(RAPID_LINE_TO_PARENT_LINE_MAP);
    let mut display_board = DisplayBoard::new();
    loop {
        update_display_board(&mut display_board, &client, &rapid_line_to_parent_map).await;
        execute!(io::stdout(), Clear(ClearType::All))?;
        console.render(&display_board).unwrap();
        if display_board.last_request_successful {
            thread::sleep(time::Duration::from_secs(30));
        } else {
            thread::sleep(time::Duration::from_secs(2));
        }
    }
}

async fn update_display_board(
    display_board: &mut DisplayBoard,
    client: &Client,
    rapid_line_to_parent_map: &HashMap<&str, &str>,
) {
    if let Ok(stops) = get_stops(client).await {
        if let Ok(display_lines) = get_display_lines(stops, rapid_line_to_parent_map, Some(true)) {
            (*display_board).display_lines = Some(display_lines);
            (*display_board).last_successful_request_time = Some(Local::now());
            (*display_board).last_request_successful = true;
        } else {
            (*display_board).last_request_successful = false;
        }
    } else {
        (*display_board).last_request_successful = false;
    }
}

fn get_display_lines(
    stops: HashMap<LineStop, Vec<MonitoredVehicleJourney>>,
    rapid_line_to_parent_map: &HashMap<&str, &str>,
    use_long_name: Option<bool>,
) -> Result<HashMap<String, Vec<MonitoredVehicleJourney>>, reqwest::Error> {
    let mut display: HashMap<String, Vec<MonitoredVehicleJourney>> = HashMap::new();

    for (line_stop, value) in stops.into_iter() {
        // let parent_line = match rapid_line_to_parent_map.get(line_stop.line_ref.as_str()) {
        //     Some(&parent_line) => line_stop.line_ref.clone(),//parent_line.to_string(),
        //     None => line_stop.line_ref.clone(),
        // };

        let parent_line = match rapid_line_to_parent_map.get(line_stop.line_ref.as_str()) {
            Some(&parent_line) => line_stop.screen_display(),
            None => line_stop.screen_display(),
        };

        //add new time to arrivals or create a new entry
        //in display lines
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
    return Ok(display);
}
