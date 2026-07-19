use serde::{Deserialize, Serialize};
pub mod gemini;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Message {
    pub role: Role,
    pub parts: Vec<Part>,
}

impl Message {
    pub fn new(role: Role, parts: Vec<Part>) -> Self {
        Self { role, parts }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Part {
    Text(String),
    InlineData,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialization() {
        let text = Part::Text(String::from("Hello everynyan!"));
        let role = Role::User;
        let message = Message::new(role, vec![text]);

        let json = serde_json::to_string(&message).unwrap();

        assert_eq!(
            json,
            r#"{"role":"user","parts":[{"text":"Hello everynyan!"}]}"#
        );
    }
}
