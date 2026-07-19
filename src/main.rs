use crate::config::Config;
use crate::provider::{
    Message, Part, Role,
    gemini::{GeminiRequest, SystemInstruction},
};
use clap::{Parser, Subcommand};
use reqwest::Client;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;

mod config;
mod provider;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// API key
    #[arg(short, long)]
    api_key: Option<String>,
    /// Config file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Send message to cli-chan
    Send {
        /// Chose which anime girl you'll send the message to (noGirl to ignore config)
        anime_girl: String,
        /// The string which you will send to the anime girl
        text: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let api_key = if let Some(key) = cli.api_key {
        key
    } else if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        key
    } else {
        return Err("Please provide an API key, pls".into());
    };

    let config_path = if let Some(path) = cli.config {
        path
    } else if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        let mut path = PathBuf::from_str(&path)?;
        path.push("cli-chan/config.toml");
        path
    } else {
        return Err("Please provide a config location".into());
    };

    let config = match fs::read_to_string(config_path).await {
        Ok(string) => string,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                return Err("You haven't created that config file did you?".into());
            }
            _ => return Err(error.into()),
        },
    };
    let config = toml::from_str::<Config>(&config)?;

    let client = Client::new();

    let Commands::Send { anime_girl, text } = cli.command;
    let part = Part::Text(text);
    let message = Message::new(Role::User, vec![part]);

    let gemini_request = GeminiRequest::new(
        vec![message],
        if anime_girl != "noGirl" {
            Some(SystemInstruction::from_str(
                config
                    .personality
                    .get(&anime_girl)
                    .ok_or("You haven't declared that Anime Girl in config file did you?")?
                    .summary(),
            ))
        } else {
            None
        },
    );

    let gemini_response = gemini_request.send(&client, api_key).await?;

    let response_text = gemini_response.get_text();

    println!("{response_text}");

    Ok(())
}
