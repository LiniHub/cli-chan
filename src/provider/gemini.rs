use crate::Message;
use crate::Part;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const MODEL: &str = "gemini-2.5-flash-lite";

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Message>,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

impl GeminiResponse {
    pub fn get_text(&self) -> Option<String> {
        let part = self.candidates.first()?.content.parts.first()?;
        match part {
            Part::Text(text) => Some(text.clone()),
            Part::InlineData => None,
        }
    }
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: Message,
}

impl GeminiRequest {
    pub async fn send(&self, client: &Client, api_key: String) -> reqwest::Result<GeminiResponse> {
        let url = format!("{BASE_URL}/{MODEL}:generateContent");
        let response = client
            .post(&url)
            .header("X-goog-api-key", api_key)
            .json(self)
            .send()
            .await?;
        response.json().await
    }
}
