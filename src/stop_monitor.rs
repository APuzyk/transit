use chrono;
use chrono::DateTime;
use reqwest;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

const STOPS_TO_MONITOR: [&str; 3] = [
    "15482", // NAPLES AND ROLPH SOUTH BOUND
    // "14891", // GENEVA AND NAPLES EAST BOUND
    "14890", // GENEVA AND NAPLES WEST BOUND
    "15593", // GENEVA AND MISSION SOUTH BOUND
];

#[derive(Deserialize, Debug)]
pub struct MonitoredVehicleJourney {
    pub line_ref: String,
    line_name: String,
    origin_name: String,
    destination_name: String,
    vehicle_location: Location,
    monitored_call: MonitoredCall,
}

impl MonitoredVehicleJourney {
    // get time to arrival in minutes
    pub fn time_to_arrival(&self) -> Option<i64> {
        let arrival_time = DateTime::parse_from_rfc3339(&self.monitored_call.expected_arrival_time);
        let now = chrono::offset::Local::now();
        let output: Option<i64> = match arrival_time {
            Ok(value) => Some(value.signed_duration_since(now).num_minutes()),
            Err(_) => None,
        };
        return output;
    }

    pub fn has_location(&self) -> bool {
        return !self.vehicle_location.is_empty();
    }
}

#[derive(Deserialize, Debug)]
pub struct Location {
    longitude: String,
    latitude: String,
}

impl Location {
    pub fn is_empty(&self) -> bool {
        return self.longitude.is_empty() || self.latitude.is_empty();
    }
}

#[derive(Deserialize, Debug)]
pub struct MonitoredCall {
    stop_point_ref: String,
    destination_display: String,
    expected_arrival_time: String,
    stop_name: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct LineStop {
    pub line_ref: String,
    line_name: String,
    origin_name: String,
    pub destination_name: String,
    stop_name: String,
}

impl LineStop {
    pub fn screen_display(&self) -> String {
        return self.line_ref.clone() + " - " + &self.destination_name;
    }
}

pub async fn get_stops(
    client: &Client,
) -> Result<HashMap<LineStop, Vec<MonitoredVehicleJourney>>, reqwest::Error> {
    let mut hm: HashMap<LineStop, Vec<MonitoredVehicleJourney>> = HashMap::new();
    for stop_id in STOPS_TO_MONITOR {
        let stop_monitor_data: Value = get_stop_monitor_request(client, stop_id).await?;
        let monitored_vehicle_journeys = extract_monitored_vehicle_journeys(stop_monitor_data);
        for mvj in monitored_vehicle_journeys {
            let line = LineStop {
                line_ref: mvj.line_ref.clone(),
                line_name: mvj.line_name.clone(),
                origin_name: mvj.origin_name.clone(),
                destination_name: mvj.destination_name.clone(),
                stop_name: mvj.monitored_call.stop_name.clone(),
            };
            if let Some(x) = hm.get_mut(&line) {
                x.push(mvj);
            } else {
                hm.insert(line, vec![mvj]);
            }
        }
    }
    return Ok(hm);
}

fn extract_monitored_vehicle_journeys(stop_monitor_data: Value) -> Vec<MonitoredVehicleJourney> {
    let mut monitored_vehicle_journeys: Vec<MonitoredVehicleJourney> = Vec::new();
    for elem in stop_monitor_data["ServiceDelivery"]["StopMonitoringDelivery"]["MonitoredStopVisit"]
        .as_array()
        .unwrap()
    {
        let mvj = elem["MonitoredVehicleJourney"].as_object().unwrap();
        let expected_arrival =
            mvj["MonitoredCall"].as_object().unwrap()["ExpectedArrivalTime"].as_str();

        if let Some(ea) = expected_arrival {
            monitored_vehicle_journeys.push(MonitoredVehicleJourney {
                line_ref: mvj["LineRef"].as_str().unwrap().to_string(),
                line_name: mvj["PublishedLineName"].as_str().unwrap().to_string(),
                origin_name: mvj["OriginName"].as_str().unwrap().to_string(),
                destination_name: mvj["DestinationName"].as_str().unwrap().to_string(),
                vehicle_location: Location {
                    latitude: mvj["VehicleLocation"].as_object().unwrap()["Latitude"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    longitude: mvj["VehicleLocation"].as_object().unwrap()["Longitude"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                },
                monitored_call: MonitoredCall {
                    stop_point_ref: mvj["MonitoredCall"].as_object().unwrap()["StopPointRef"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    destination_display: mvj["MonitoredCall"].as_object().unwrap()
                        ["DestinationDisplay"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    expected_arrival_time: ea.to_string(),
                    stop_name: mvj["MonitoredCall"].as_object().unwrap()["StopPointName"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                },
            });
        }
    }
    return monitored_vehicle_journeys;
}

async fn get_stop_monitor_request(client: &Client, stop_id: &str) -> Result<Value, reqwest::Error> {
    let token = env::var("TRANSIT_TOKEN").unwrap();
    let url: String = "https://api.511.org/transit/StopMonitoring?api_key=".to_owned()
        + &token
        + "&agency=SF&stopCode="
        + stop_id;
    let response = client.get(url).send().await.unwrap();

    let response_body = response.error_for_status()?.text().await?;
    let data: Value = serde_json::from_str(response_body.as_str()).unwrap();
    return Ok(data);
}
