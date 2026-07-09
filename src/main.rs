use crate::provider::gemini::GeminiRequest;
use crate::provider::Message;
use crate::provider::Part;
use crate::provider::Role;
use clap::Parser;
use clap::Subcommand;
use reqwest::Client;
use std::error::Error;
mod provider;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// API key
    #[arg(short, long)]
    api_key: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Send message to cli-chan
    Send { text: String },
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

    let client = Client::new();

    let Commands::Send { text } = cli.command;
    let part = Part::Text(text);
    let message = Message::new(Role::User, vec![part]);

    let gemini_request = GeminiRequest {
        contents: vec![message],
    };

    let gemini_response = gemini_request.send(&client, api_key).await?;

    let response_text = gemini_response.get_text().unwrap();

    println!("{response_text}");

    Ok(())
}
