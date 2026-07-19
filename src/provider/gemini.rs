use crate::{Message, Part};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const MODEL: &str = "gemini-2.5-flash-lite";

#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
}

#[derive(Debug, Serialize)]
pub struct SystemInstruction {
    pub parts: Vec<Part>,
}

impl SystemInstruction {
    pub fn from_str(instruction: String) -> Self {
        Self {
            parts: vec![Part::Text(instruction)],
        }
    }
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    candidates: Vec<Candidate>,
}

impl GeminiResponse {
    pub fn get_text(&self) -> String {
        let part = &self.candidates[0].content.parts[0];
        match part {
            Part::Text(text) => text.clone(),
            Part::InlineData => unimplemented!(), // maybe in future
        }
    }
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: Message,
}

impl GeminiRequest {
    pub fn new(contents: Vec<Message>, system_instruction: Option<SystemInstruction>) -> Self {
        Self {
            contents,
            system_instruction,
        }
    }

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::provider::Role;

    #[test]
    fn gemini_request_parsing() {
        let gemini_request = GeminiRequest {
            contents: vec![Message {
                role: Role::User,
                parts: vec![Part::Text("Who are you?".into())],
            }],
            system_instruction: Some(
                SystemInstruction::from_str(
                    "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."
                        .into()
                )
            ),
        };
        let json = serde_json::to_string(&gemini_request).unwrap();
        assert_eq!(
            json,
            r#"{"contents":[{"role":"user","parts":[{"text":"Who are you?"}]}],"system_instruction":{"parts":[{"text":"You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."}]}}"#
        );
    }

    #[test]
    fn gemini_request_parsing_no_instruction() {
        let gemini_request = GeminiRequest {
            contents: vec![Message {
                role: Role::User,
                parts: vec![Part::Text("Who are you?".into())],
            }],
            system_instruction: None,
        };
        let json = serde_json::to_string(&gemini_request).unwrap();
        assert_eq!(
            json,
            r#"{"contents":[{"role":"user","parts":[{"text":"Who are you?"}]}]}"#
        );
    }
}
