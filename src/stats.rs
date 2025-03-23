use serde::Deserialize;
use reqwest::blocking::get;
use std::error::Error;
use ratatui::{
    
    style::{Color, Style},
    text::{Span, Spans},
    
};

#[derive(Deserialize, Debug)]
pub struct Stats {
    #[serde(rename = "difficulty")]
    pub difficulty: u64,
    #[serde(rename = "height")]
    pub height: u64,
    #[serde(rename = "hashrate")]
    pub hashrate: f64,
    #[serde(rename = "total_emission", deserialize_with = "deserialize_total_emission")]
    pub total_emission: u128,
    #[serde(rename = "last_reward")]
    pub last_reward: u64,
    #[serde(rename = "last_timestamp")]
    pub last_timestamp: u64,
}

// Custom deserialization function for total_emission
fn deserialize_total_emission<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Deserialize the field as a string
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    // Parse the string into a u128
    s.parse::<u128>().map_err(serde::de::Error::custom)
}

impl Stats {
    // Create a new Stats instance with default values
    pub fn new() -> Self {
        Stats {
            difficulty: 0,
            height: 0,
            hashrate: 0.0,
            total_emission: 0,
            last_reward: 0,
            last_timestamp: 0,
        }
    }

    // Update the Stats instance with new data from the API
    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let response = get("https://localmonero.co/blocks/api/get_stats")?;
        let raw_json = response.text()?;
        

        // Deserialize the JSON into the Stats struct
        let stats: Stats = serde_json::from_str(&raw_json)?;
        

        self.difficulty = stats.difficulty;
        self.height = stats.height;
        self.hashrate = stats.hashrate;
        self.total_emission = stats.total_emission;
        self.last_reward = stats.last_reward;
        self.last_timestamp = stats.last_timestamp;

        Ok(())
    }

    // Display the stats
    pub fn display(&self) {
        println!("Difficulty: {}", self.difficulty);
        println!("Height: {}", self.height);
        println!("Hashrate: {:.2} H/s", self.hashrate);
        println!("Current Emission: {}", self.total_emission);
        println!("Last Reward: {}", self.last_reward);
        println!("Last Timestamp: {}", self.last_timestamp);
    }
    pub fn to_spans(&self) -> Vec<Spans> {
        vec![
            Spans::from(vec![
                Span::raw("Difficulty: "),
                Span::styled(self.difficulty.to_string(), Style::default().fg(Color::Yellow)),
            ]),
            Spans::from(vec![
                Span::raw("Height: "),
                Span::styled(self.height.to_string(), Style::default().fg(Color::Yellow)),
            ]),
            Spans::from(vec![
                Span::raw("Hashrate: "),
                Span::styled(format!("{:.2} H/s", self.hashrate), Style::default().fg(Color::Yellow)),
            ]),
            Spans::from(vec![
                Span::raw("Total Emission: "),
                Span::styled(self.total_emission.to_string(), Style::default().fg(Color::Yellow)),
            ]),
            Spans::from(vec![
                Span::raw("Last Reward: "),
                Span::styled(self.last_reward.to_string(), Style::default().fg(Color::Yellow)),
            ]),
            Spans::from(vec![
                Span::raw("Last Timestamp: "),
                Span::styled(self.last_timestamp.to_string(), Style::default().fg(Color::Yellow)),
            ]),
        ]
    }
}